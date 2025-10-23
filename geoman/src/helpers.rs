pub fn get_configuration_directory() -> std::path::PathBuf {
    let base_path = std::env::var("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .map(|p| {
            p.parent()
                .expect("Failed to get parent directory")
                .to_path_buf()
        })
        .unwrap_or_else(|_| {
            std::env::current_dir().expect("Failed to determine current directory")
        });
    base_path.join("configuration")
}
