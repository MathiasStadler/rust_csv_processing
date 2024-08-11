# project init

- https://stackoverflow.com/questions/5130847/running-multiple-commands-in-one-line-in-shell
- && executes the right-hand command of && only if the previous one succeeded

- change before the rust_project name

```bash
cd \
&& mkdir rust_project \
&& cd $_ \
&& touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show 
# cold update 
# https://github.com/rust-lang/rustup/issues/2729
# rustup toolchain uninstall stable
# rustup toolchain install stable
rustup show
touch FROM_HERE.md 
cargo build
```

## package for this/these project(s)

```bash
# inside project folder
cargo add serde_json
```
