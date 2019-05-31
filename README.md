# ros

A tiny OS based on https://os.phil-opp.com

# How to build and run

```
rustup component add llvm-tools-preview
rustup component add rust-src
cargo install cargo-xbuild
cargo install bootimage
# Probably, one should first build the bootloader
bootimage build
# PATH to QEMU is to be set
cargo xrun
```

## VSCode settings

.vscode/settings.json should be something like

```json
{
    "rust.target": "x86_64-ros.json",
    "rust.sysroot": "PROJECT_ROOT/target/sysroot"
}
```

# (Other) Useful links

[OSDev](https://wiki.osdev.org)

[IA32 Instructions (in Japanese)](http://softwaretechnique.jp/OS_Development/Tips/IA32_instructions.html)

[x86 and amd64 instruction reference](https://www.felixcloutier.com/x86/)
