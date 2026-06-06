// main.rs
mod gui_module;
mod tui_module;
mod network_lib;
mod html_parser;

use std::sync::Arc;
use tokio::runtime::Runtime;
use tauri::{App, Manager};
use gui_module::GUIHandler;
use tui_module::TUIHandler;
use network_lib::Browser;

fn main() {
    let app = App::new().setup(|webview, _| {
        // Initialize the GUI and TUI handlers
        let gui_handler = GUIHandler::new();
        let tui_handler = TUIHandler::new();
        
        // Create a shared runtime for tokio
        let rt = Arc::new(Runtime::new().unwrap());

        // Spawn tasks in the runtime
        rt.spawn(async move {
            let browser = Browser::new();
            let url = "https://example.com".to_string();
            let content = browser.navigate(url).await;
            
            // Update GUI and TUI with fetched content
            gui_handler.display_content(content);
            tui_handler.render_content(content);
        });

        // Set up the webview for Tauri
        let window = webview.window().build()?;
        Ok(())
    });
}
