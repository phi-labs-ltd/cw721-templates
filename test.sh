cd ../
cargo generate --path ./nft-token-template --name updatable --define token_type=updatable
cd ./updatable
cargo build

cd ../
cargo generate --path ./nft-token-template --name non-updatable --define token_type=non-updatable
cd ./non-updatable
cargo build