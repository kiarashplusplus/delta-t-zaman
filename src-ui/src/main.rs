mod ipc;
mod models;
mod state;
mod utils;
mod daynight;
mod components {
    pub mod clock;
    pub mod alarms;
    pub mod settings;
    pub mod timezone_search;
    pub mod alarm_form;
    pub mod ui;
}
mod app;

use leptos::*;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
