use leptos::prelude::*;
use leptos_demo::App;
use log::Level;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("It should work");
    mount_to_body(App);
}
