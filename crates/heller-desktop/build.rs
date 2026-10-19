fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").ok_or("Cargo did not supply OUT_DIR")?,
    )
    .join("translations");
    for lang in ["en", "ru", "de", "es", "fr", "pt", "it", "zh", "ja"] {
        let dest = root.join(lang).join("LC_MESSAGES");
        std::fs::create_dir_all(&dest)?;
        std::fs::copy(
            format!("../heller-ui/lang/{lang}.po"),
            dest.join("heller-desktop.po"),
        )?;
        println!("cargo:rerun-if-changed=../heller-ui/lang/{lang}.po");
    }
    slint_build::compile_with_config(
        "ui/tray.slint",
        slint_build::CompilerConfiguration::new()
            .with_default_translation_context(slint_build::DefaultTranslationContext::None)
            .with_bundled_translations(root),
    )?;
    Ok(())
}
