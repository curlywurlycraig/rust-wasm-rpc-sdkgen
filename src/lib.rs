pub use remote_attr::remote;

pub use bincode;

#[cfg(feature = "frontend")]
pub use {
    web_sys,
    js_sys
};
