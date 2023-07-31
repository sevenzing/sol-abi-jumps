Solidity ABI jumps
===

Parses all public solidity function (ABI functions) and finds its location in solidity sources

## How to run

```console
cargo run -- --contract <path to contract/directory> --name <contract name> --solc-ver <solidity version>
```

## Examples

```console
cargo run -- --contract ./contract --name MainContract --solc-ver 0.8.9
func_with_args(uint256)
./contract/main.sol:9

my_id()
./contract/main.sol:5

owner()
./contract/owner.sol:17

renounceOwnership()
./contract/owner.sol:35

transferOwnership(address)
./contract/owner.sol:43
```

```console
cargo run -- -c ./tests/contracts/token --name TestToken --solc-ver 0.8.17
allowance(address,address)
./tests/contracts/token/main.sol:336

alwaysReverts()
./tests/contracts/token/main.sol:610

approve(address,uint256)
./tests/contracts/token/main.sol:350

balanceOf(address)
./tests/contracts/token/main.sol:315

decimals()
./tests/contracts/token/main.sol:301

decreaseAllowance(address,uint256)
./tests/contracts/token/main.sol:415

increaseAllowance(address,uint256)
./tests/contracts/token/main.sol:395

mint(address,uint256)
./tests/contracts/token/main.sol:606

name()
./tests/contracts/token/main.sol:276

owner()
./tests/contracts/token/main.sol:62

renounceOwnership()
./tests/contracts/token/main.sol:81

symbol()
./tests/contracts/token/main.sol:284

totalSupply()
./tests/contracts/token/main.sol:308

transfer(address,uint256)
./tests/contracts/token/main.sol:327

transferFrom(address,address,uint256)
./tests/contracts/token/main.sol:372

transferOwnership(address)
./tests/contracts/token/main.sol:89
```
