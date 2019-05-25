# ros

A tiny OS based on https://os.phil-opp.com

# How to build and run

```
cargo install cargo-xbuild
cargo install bootimage
# PATH to QEMU is to be set
cargo xrun
```

## VSCode settings

.vscode/settings.json should be something like

```json
{
    "rust.target": "x86_64-ros.json",
    "rust.sysroot": "project_root/sysroot"
}
```

# (Other) Useful links

[OSDev](https://wiki.osdev.org)

[IA32 Instructions (in Japanese)](http://softwaretechnique.jp/OS_Development/Tips/IA32_instructions.html)

[x86 and amd64 instruction reference](https://www.felixcloutier.com/x86/)
