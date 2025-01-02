// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto_price;

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_set_price({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            match crypto_price::get_bitcoin_price() {
                Ok(price) => {
                    let price = format!("{:.2}", price); // returns "1.24"
                    ui.set_price(slint::SharedString::from(price).clone());
                }
                Err(_) => {
                    println!("Failed to grab price");
                }
            }
        }
    });

    ui.run()?;

    Ok(())
}
