# project init

```bash
touch README.md
ln -s README.md README
cargo init .
cargo add rustfmt
rustup component add rustfmt
mkdir examples
cp src/main.rs examples/example.rs
sed -i -e 's/world/example/g' examples/example.rs
rustup  show
rustup  check
rustup update  --force
# cold update 
# https://github.com/rust-lang/rustup/issues/2729
# rustup toolchain uninstall stable
# rustup toolchain install stable

rustup show
```

<!-- chmod -R 0777 /opt/rust
rm /opt/rust/bin/rust-analyzer 
rm /opt/rust/bin/rustfmt 
rm /opt/rust/bin/cargo-fmt
rustup show

rustup update -->



touch FROM_HERE.md 

cargo build