use crate::opcodes::{opcode, Opcode};
use ethers::types::Bytes;

pub struct DisassembledOpcode {
    pub operation: Opcode,
    pub program_counter: usize,
    pub args: Vec<u8>,
}

// Changed version of
// https://github.com/Jon-Becker/heimdall-rs/blob/6363d2fe02b68a4b03e0d5f726f605d1360250b7/common/src/ether/evm/ext/disassemble.rs#L34
pub fn disassemble_bytecode(bytecode: &Bytes) -> Vec<DisassembledOpcode> {
    let mut program_counter = 0;
    let mut output = Vec::new();

    // Iterate over the bytecode, disassembling each instruction.
    while program_counter < bytecode.len() {
        let operation = opcode(bytecode[program_counter]);
        let args = if operation.name.contains("PUSH") {
            let byte_count_to_push: u8 = operation
                .name
                .replace("PUSH", "")
                .parse()
                .expect("PUSH should contain number");

            match bytecode
                .get(program_counter + 1..program_counter + 1 + byte_count_to_push as usize)
            {
                Some(bytes) => bytes.to_vec(),
                None => vec![],
            }
        } else {
            vec![]
        };
        let opcode = DisassembledOpcode {
            operation,
            program_counter,
            args,
        };
        program_counter += opcode.args.len();
        program_counter += 1;
        output.push(opcode);
    }

    output
}
