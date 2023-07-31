use ethers_solc::{CompilerInput, EvmVersion, Solc};
use pretty_assertions::assert_eq;
use sha3::{Digest, Keccak256};
use sol_abi_jumps::{find_solidity_methods_in_output, SolidityMethod};

fn solidity_method(sig: &str, filename: &str, line_no: usize) -> SolidityMethod {
    let data: Vec<u8> = sig.into();
    let mut selector = [0u8; 4];
    selector.copy_from_slice(&Keccak256::digest(data)[..4]);

    SolidityMethod {
        sig: sig.to_string(),
        selector,
        filename: filename.to_string(),
        line_no,
    }
}

#[test]
fn single_file() {
    let (contract_name, dir, version) = ("TestToken", "tests/contracts/token", "0.8.17");

    let solc = Solc::find_or_install_svm_version(version).expect("failed to install version");

    let inputs = CompilerInput::new(dir).expect("failed to read dir");
    let input = inputs[0].clone().evm_version(EvmVersion::London);
    let output = solc.compile(&input).expect("failed to compile");
    assert!(
        !output.contracts.is_empty(),
        "errors during compilation: {:?}",
        output.errors
    );
    let solidity_methods = find_solidity_methods_in_output(&input, &output, contract_name)
        .expect("failed to find solidity methods");

    let method =
        |sig, line_no: usize| solidity_method(sig, "tests/contracts/token/main.sol", line_no);
    let expected = vec![
        method("allowance(address,address)", 336),
        method("alwaysReverts()", 610),
        method("approve(address,uint256)", 350),
        method("balanceOf(address)", 315),
        method("decimals()", 301),
        method("decreaseAllowance(address,uint256)", 415),
        method("increaseAllowance(address,uint256)", 395),
        method("mint(address,uint256)", 606),
        method("name()", 276),
        method("owner()", 62),
        method("renounceOwnership()", 81),
        method("symbol()", 284),
        method("totalSupply()", 308),
        method("transfer(address,uint256)", 327),
        method("transferFrom(address,address,uint256)", 372),
        method("transferOwnership(address)", 89),
    ];

    assert_eq!(solidity_methods, expected);
}
