mod framework;

pub fn log_print(text: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&text.into());

    #[cfg(not(target_arch = "wasm32"))]
    println!("{text}");
}