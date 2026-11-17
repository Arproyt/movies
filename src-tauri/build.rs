fn main() {
    let mut attributes = tauri_build::Attributes::new();

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let icon_path =
            std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR is missing"))
                .join("app.ico");

        let mut icon = Vec::with_capacity(70);
        icon.extend_from_slice(&[0, 0, 1, 0, 1, 0]);
        icon.extend_from_slice(&[1, 1, 0, 0]);
        icon.extend_from_slice(&1u16.to_le_bytes());
        icon.extend_from_slice(&32u16.to_le_bytes());
        icon.extend_from_slice(&48u32.to_le_bytes());
        icon.extend_from_slice(&22u32.to_le_bytes());
        icon.extend_from_slice(&40u32.to_le_bytes());
        icon.extend_from_slice(&1i32.to_le_bytes());
        icon.extend_from_slice(&2i32.to_le_bytes());
        icon.extend_from_slice(&1u16.to_le_bytes());
        icon.extend_from_slice(&32u16.to_le_bytes());
        icon.extend_from_slice(&0u32.to_le_bytes());
        icon.extend_from_slice(&4u32.to_le_bytes());
        icon.extend_from_slice(&[0; 16]);
        icon.extend_from_slice(&[24, 24, 24, 255]);
        icon.extend_from_slice(&[0; 4]);

        std::fs::write(&icon_path, icon).expect("Failed to write Windows icon");

        attributes = attributes
            .windows_attributes(tauri_build::WindowsAttributes::new().window_icon_path(icon_path));
    }

    tauri_build::try_build(attributes).expect("Tauri build failed");
}
