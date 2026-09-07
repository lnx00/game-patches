fn main() {
    if std::env::var("TARGET").unwrap_or_default().contains("windows") {
        let mut res = winres::WindowsResource::new();

        let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
        res.set("OriginalFilename", &format!("{pkg_name}.asi"));

        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {e}");
            std::process::exit(1);
        }
    }
}