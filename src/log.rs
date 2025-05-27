// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        #[cfg(target_family = "wasm")]
        web_sys::console::log_1(&format!( $( $t )* ).into());

        #[cfg(not(target_family = "wasm"))]
        {}
    };
}

pub(crate) use log;
