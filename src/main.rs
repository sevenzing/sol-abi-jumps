use clap::Parser;
use ethers_solc::{CompilerInput, EvmVersion, Solc};
use sol_abi_jumps::find_solidity_methods_in_output;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Contract path
    #[arg(short, long)]
    contract: PathBuf,
    #[arg(short, long, default_value = "0.8.9")]
    solc_ver: String,

    #[arg(short, long, default_value = "MainContract")]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = Args::parse();
    if !args.contract.exists() {
        panic!("contract not exists");
    }
    let solc =
        Solc::find_or_install_svm_version(&args.solc_ver).expect("failed to install version");

    let inputs = CompilerInput::new(args.contract)?;
    let input = inputs[0].clone().evm_version(EvmVersion::London);
    let output = solc.async_compile(&input).await?;

    let solidity_methods = find_solidity_methods_in_output(&input, &output, &args.name)?;

    for method in solidity_methods {
        println!("{}", method.sig);
        println!("{}:{}\n", method.filename, method.line_no);
    }
    Ok(())
}
