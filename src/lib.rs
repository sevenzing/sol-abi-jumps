use disassemble::DisassembledOpcode;
use ethers_solc::{
    artifacts::{Source, SourceFile},
    sourcemap::SourceElement,
    CompilerInput, CompilerOutput,
};
use std::{collections::BTreeMap, path::PathBuf};

pub mod disassemble;
pub mod opcodes;

pub fn find_solidity_methods_in_output(
    input: &CompilerInput,
    output: &CompilerOutput,
    contract_name: &str,
) -> Result<Vec<SolidityMethod>, anyhow::Error> {
    let (_, contract) = output
        .contracts_iter()
        .find(|(name, _)| *name == contract_name)
        .ok_or_else(|| anyhow::anyhow!("contract {contract_name} not found"))?;
    let evm = contract.evm.as_ref().expect("evm included");
    let deployed_bytecode = evm
        .deployed_bytecode
        .as_ref()
        .expect("deployde bytecode included");
    let bytecode = deployed_bytecode
        .bytecode
        .as_ref()
        .expect("bytecode included");
    let methods = parse_selectors(evm.method_identifiers.clone())?;
    let source_map = bytecode.source_map().expect("srcmap included")?;
    let bytecode_raw = bytecode.object.as_bytes().expect("bytecode included");
    let opcodes = disassemble::disassemble_bytecode(bytecode_raw);

    find_solidity_methods(
        methods,
        &source_map,
        &opcodes,
        &output.sources,
        &input.sources,
    )
}

fn parse_selectors(methods: BTreeMap<String, String>) -> anyhow::Result<BTreeMap<String, [u8; 4]>> {
    let methods: BTreeMap<String, [u8; 4]> = methods
        .into_iter()
        .map(|(name, selector)| {
            let mut result = [0u8; 4];
            hex::decode_to_slice(selector, &mut result).map(|_| (name, result))
        })
        .collect::<Result<_, _>>()?;
    Ok(methods)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolidityMethod {
    pub sig: String,
    pub selector: [u8; 4],
    pub filename: String,
    pub line_no: usize,
}

impl SolidityMethod {
    pub fn new(
        sig: String,
        selector: [u8; 4],
        src: SourceElement,
        sources_input: &BTreeMap<PathBuf, Source>,
        sources_output: &BTreeMap<String, SourceFile>,
    ) -> anyhow::Result<Self> {
        let (filename, _) = src
            .index
            .and_then(|file_id| sources_output.iter().find(|(_, file)| file.id == file_id))
            .ok_or_else(|| anyhow::anyhow!("src not found in outputed sources"))?;
        let content = sources_input
            .get(&PathBuf::from(filename))
            .ok_or_else(|| anyhow::anyhow!("src not found in inputed sources"))?;

        let start = src.offset;
        let line_no = content.content[..start]
            .chars()
            .filter(|c| *c == '\n')
            .count()
            + 1;
        Ok(Self {
            sig,
            selector,
            filename: filename.to_string(),
            line_no,
        })
    }
}

pub fn find_solidity_methods(
    methods: BTreeMap<String, [u8; 4]>,
    source_map: &[SourceElement],
    opcodes: &[DisassembledOpcode],
    sources_output: &BTreeMap<String, SourceFile>,
    sources_input: &BTreeMap<PathBuf, Source>,
) -> anyhow::Result<Vec<SolidityMethod>> {
    let mut result = vec![];
    for (func_sig, selector) in methods {
        let maybe_function_index = find_selector(selector, opcodes);
        if let Some(function_index) = maybe_function_index {
            tracing::info!("found function {} in {}", func_sig, function_index);
            let src = match source_map.get(function_index) {
                Some(src) => src,
                None => {
                    tracing::info!(selector =? selector, index =? function_index, "source map doesn't have function index");
                    continue;
                }
            };
            let method = SolidityMethod::new(
                func_sig,
                selector,
                src.clone(),
                sources_input,
                sources_output,
            )?;
            result.push(method);
        } else {
            tracing::warn!(
                "function {} with selector '{}' not found in bytecode",
                func_sig,
                hex::encode(selector)
            )
        }
    }

    Ok(result)
}

fn find_selector(selector: [u8; 4], opcodes: &[DisassembledOpcode]) -> Option<usize> {
    // we are looking for
    // PUSH4 <SELECTOR>
    // EQ
    // PUSH1 <JUMP_DEST>
    // JUMPI
    for window in opcodes.windows(4) {
        if window[0].operation.name == "PUSH4"
            && window[0].args == selector
            && window[1].operation.name == "EQ"
            && window[2].operation.name.starts_with("PUSH")
            && window[3].operation.name == "JUMPI"
        {
            let jump_to =
                usize::from_str_radix(&hex::encode(&window[2].args), 16).expect("valid hex string");

            let maybe_target_opcode_index = opcodes
                .iter()
                .enumerate()
                .find_map(|(index, opcode)| (opcode.program_counter == jump_to).then_some(index));

            match maybe_target_opcode_index {
                Some(index) => return Some(index),
                None => tracing::warn!(selector =? selector, "target opcode not found"),
            }
        }
    }

    None
}
