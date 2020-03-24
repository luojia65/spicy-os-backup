// NOTE: Adapted from riscv-rt/build.rs

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/boot/linker32.ld");
}
