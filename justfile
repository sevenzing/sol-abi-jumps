file_name := "storage.sol"
output_dir := "output"

ast:
    for version in "v0.4.21" "v0.5.17" "v0.6.12" "v0.7.6" "v0.8.0"; do \
        ./compilers/solc-${version} --combined-json ast,compact-format,hashes {{file_name}} | jq > {{output_dir}}/${version}.json; \
    done

    ./compilers/solc-v0.8.20 --combined-json ast {{file_name}} | jq > {{output_dir}}/v0.8.20.json

