fn main() {
    println!("cargo::rustc-check-cfg=cfg(built)");
    println!("cargo::rustc-cfg=built");
}
