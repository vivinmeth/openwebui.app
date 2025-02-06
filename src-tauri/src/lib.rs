// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//

use std::any::Any;

use tauri::{ipc::IpcResponse, menu::*, Manager};

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
            let home_menu_item = MenuItem::new(handle, "Home", true, None::<&str>)?;
            let home_menu_item_id = home_menu_item.id().clone();
            let dashboard_menu_item = MenuItem::new(handle, "Dashboard", true, None::<&str>)?;
            let dashboard_menu_item_id = dashboard_menu_item.id().clone();
            let sub_menu = SubmenuBuilder::new(handle, "Nav")
                .items(&[&home_menu_item, &dashboard_menu_item])
                .build()?;

            menu.append(&sub_menu)?;
            let _ = app.set_menu(menu);

            app.on_menu_event(move |app, ev| {
                let main_window = app.get_webview_window("main").unwrap();
                if *ev.id() == home_menu_item_id {
                    main_window.eval(&format!(
                        "window.location.replace('http://localhost:{}')",
                        "3000"
                    ));
                } else if *ev.id() == dashboard_menu_item_id {
                    main_window.eval(&format!(
                        "window.location.replace('http://localhost:{}')",
                        "7878"
                    ));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
