# project init

touch README.md
ln -s README.md README
cargo init .
cargo add rustfmt
rustup component add rustfmt
mkdir examples
cp src/main.rs examples/example.rs
sed -i -e 's/world/example/g' examples/example.rs
cargo build