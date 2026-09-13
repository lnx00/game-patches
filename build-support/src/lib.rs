use std::env;

pub use winresource;

pub fn create_windows_resource() -> winresource::WindowsResource {
    let mut res = winresource::WindowsResource::new();

    let pkg_name = env::var("CARGO_PKG_NAME").unwrap_or_default();
    let pkg_desc = env::var("CARGO_PKG_DESCRIPTION").unwrap_or_else(|_| pkg_name.clone());
    let pkg_authors = env::var("CARGO_PKG_AUTHORS")
        .map(|a| a.replace(':', ", "))
        .unwrap_or_default();

    let file_name = format!("{}.asi", pkg_name.replace('-', "_"));

    // Set filetype to VFT_DLL (0x2)
    res.set_version_info(winresource::VersionInfo::FILETYPE, 0x00000002);

    res.set("OriginalFilename", &file_name)
        .set("InternalName", &file_name)
        .set("FileDescription", &pkg_desc)
        .set("ProductName", &pkg_name);

    if !pkg_authors.is_empty() {
        res.set("CompanyName", &pkg_authors);
    }

    res
}

pub fn setup_windows_resources() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let res = create_windows_resource();

        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {e}");
            std::process::exit(1);
        }
    }
}
