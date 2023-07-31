Solidity ABI jumps
===

Parses all public solidity function (ABI functions) and finds its location in solidity sources

## How to run

```console
cargo run -- --contract <path to contract/directory> --name <contract name> --solc-ver <solidity version>
```

for example:

```console
cargo run -- --contract ./contract --name MainContract --solc-ver 0.8.9
cargo run -- -c ./tests/contracts/token --name TestToken --solc-ver 0.8.17
```
