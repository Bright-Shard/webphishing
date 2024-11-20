fn main() {
    let path = std::fs::canonicalize(std::path::Path::new("exports.def")).unwrap();
    println!("cargo:rustc-cdylib-link-arg={}", path.display());
}
