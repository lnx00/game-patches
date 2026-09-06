fn main() {
    // Only embed resources when targeting Windows
    if std::env::var("TARGET").unwrap_or_default().contains("windows") {
        let mut res = winres::WindowsResource::new();

        // winres automatically picks up version, authors, and description from Cargo.toml.
        // You only need to explicitly set things specific to your .asi naming:
        let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
        res.set("OriginalFilename", &format!("{pkg_name}.asi"));

        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {e}");
            std::process::exit(1);
        }
    }
}