// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//

use std::str::FromStr;

use rand;
use tauri::{ipc::IpcResponse, menu::*, Manager, Url, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn _greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let menu = app.menu().expect("menu not found!");
            // let go_submenu = menu.get("Go").expect("go menu not found!");
            // println!("{:?}", go_submenu.id());
            let home_menu_item = MenuItem::new(handle, "Chat", true, None::<&str>)?;
            let home_menu_item_id = home_menu_item.id().clone();
            let dashboard_menu_item = MenuItem::new(handle, "Dashboard", true, None::<&str>)?;
            let dashboard_menu_item_id = dashboard_menu_item.id().clone();
            let sub_menu = SubmenuBuilder::new(handle, "Nav")
                .items(&[&home_menu_item, &dashboard_menu_item])
                .build()?;

            menu.append(&sub_menu)?;
            let _ = app.set_menu(menu);

            app.on_menu_event(move |app, ev| {
                let _main_window = app.get_webview_window("main").unwrap();
                let random_number = rand::random::<i64>();
                if *ev.id() == home_menu_item_id {
                    let window = WebviewWindowBuilder::new(
                        app,
                        random_number.to_string(),
                        tauri::WebviewUrl::External(
                            Url::from_str("http://localhost:3000").unwrap(),
                        ),
                    )
                    .title("OpenWebUI Dashboard")
                    .focused(true)
                    .min_inner_size(1366.0, 768.0)
                    .visible(true)
                    .build()
                    .unwrap();
                    window.show().unwrap();
                } else if *ev.id() == dashboard_menu_item_id {
                    // main_window.eval(&format!(
                    //     "window.location.replace('http://localhost:{}')",
                    //     "7878"
                    // ));

                    let window = WebviewWindowBuilder::new(
                        app,
                        random_number.to_string(),
                        tauri::WebviewUrl::External(
                            Url::from_str("http://localhost:7878/panel").unwrap(),
                        ),
                    )
                    .title("OpenWebUI Dashboard")
                    .focused(true)
                    .min_inner_size(1366.0, 768.0)
                    .visible(true)
                    .build()
                    .unwrap();
                    window.show().unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
