
cargo +nightly build --target=riscv64im-unknown-evm-elf.json  -Z build-std=alloc,core,panic_abort -Z build-std-features=compiler-builtins-mem --release