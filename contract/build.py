import json

inpt = {
    "language": "Solidity",
    "sources": {},
    "settings": {
        "outputSelection": {
            "*": {
                "": [
                    "ast"
                ],
                "*": [
                    "abi",
                    "metadata",
                    "devdoc",
                    "userdoc",
                    "storageLayout",
                    "evm.legacyAssembly",
                    "evm.bytecode",
                    "evm.deployedBytecode",
                    "evm.methodIdentifiers",
                    "evm.gasEstimates",
                    "evm.assembly"
                ]
            }
        }
    }
}

files = [
    'context.sol',
    'main.sol',
    'owner.sol',
]

for file in files:
    inpt['sources'][file] = {"content": open(file).read()}

json.dump(inpt, open('input.json', 'w'), indent=4)