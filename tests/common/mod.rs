#![allow(dead_code)]
pub fn default_user_agent() -> String {
    format!("posthog-rs/{}", env!("CARGO_PKG_VERSION"))
}

pub fn install_rustls_provider() {
    use std::sync::Once;
    static INSTALL: Once = Once::new();
    INSTALL.call_once(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("ring should be the first rustls crypto provider installed");
    });
}
