pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://7movies.ac/".parse().unwrap()),
            )
            .title("7Movies")
            .inner_size(1280.0, 800.0)
            .center()
            .theme(Some(tauri::Theme::Dark))
            .user_agent("7MoviesDesktopApp/1.0")
            .on_navigation(|url| {
                url.scheme() == "https"
                    && url
                        .host_str()
                        .is_some_and(|host| host == "7movies.ac" || host.ends_with(".7movies.ac"))
            })
            .on_new_window(|_, _| tauri::webview::NewWindowResponse::Deny)
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Failed to run 7Movies");
}
