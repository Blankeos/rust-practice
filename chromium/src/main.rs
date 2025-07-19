use civet::{App, Browser, BrowserSettings};

fn main() {
    // Initialize the CEF application
    let mut app = App::new().expect("Failed to initialize CEF app");

    // Configure the browser window settings
    let browser_settings = BrowserSettings {
        window_title: Some("My Rust Browser".to_string()),
        width: 800,
        height: 600,
        ..Default::default()
    };

    // Create a browser window with a starting URL
    let browser = Browser::new(&mut app, "https://www.example.com", browser_settings)
        .expect("Failed to create browser");

    // Run the CEF application
    app.run();
}
