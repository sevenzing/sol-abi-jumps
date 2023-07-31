Solidity ABI jumps
===

Parses all public solidity function (ABI functions) and finds its location in solidity sources

## How to run

```console
cargo run -- --contract <path to contract/directory> --name <contract name>
```

for example:

```console
cargo run -- --contract ./contract --name MainContract
cargo run -- -c ./tests/contracts/token --name TestToken
```
