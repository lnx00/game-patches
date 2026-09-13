use std::env;

pub use winresource;

const VFT_DLL: u64 = 0x2;

pub fn create_windows_resource(ext: &str) -> winresource::WindowsResource {
    let mut res = winresource::WindowsResource::new();

    let pkg_name = env::var("CARGO_PKG_NAME").unwrap_or_default();
    let pkg_desc = env::var("CARGO_PKG_DESCRIPTION").unwrap_or_else(|_| pkg_name.clone());
    let pkg_authors = env::var("CARGO_PKG_AUTHORS")
        .map(|a| a.replace(':', ", "))
        .unwrap_or_default();

    let file_name = format!("{}.{}", pkg_name.replace('-', "_"), ext);

    res.set_version_info(winresource::VersionInfo::FILETYPE, VFT_DLL);

    res.set("OriginalFilename", &file_name)
        .set("InternalName", &file_name)
        .set("FileDescription", &pkg_desc)
        .set("ProductName", &pkg_name);

    if !pkg_authors.is_empty() {
        res.set("CompanyName", &pkg_authors);
    }

    res
}

pub fn setup_windows_resources_with_ext(ext: &str) {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let res = create_windows_resource(ext);

        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {e}");
            std::process::exit(1);
        }
    }
}

pub fn setup_windows_resources() {
    setup_windows_resources_with_ext("asi");
}
