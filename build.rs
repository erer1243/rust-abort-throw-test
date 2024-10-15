fn main() {
    let status = std::process::Command::new("g++")
        .args(["-fPIC", "-shared", "-o", "target/libx.so", "src/x.cpp"])
        .status()
        .unwrap();
    assert!(status.success());
    println!("cargo::rustc-link-search=native=./target");
    println!("cargo::rustc-link-lib=dylib=x");
}
