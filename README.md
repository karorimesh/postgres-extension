# postgres-extension
A postgres extension with, row level security, grpc calls, and spice db

#Running instructions
1. Install buf CLI 
brew install bufbuild/buf/buf

2.Install cargo-make 
cargo install cargo-make

3.Regenerating the client
cargo make build

4. Run extension
cargo install cargo-pgx
If your client is pg 13 in cargo.toml file then:
cargo pgx run pg13
