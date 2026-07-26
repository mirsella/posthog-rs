use std::sync::Once;

pub fn install_rustls_provider() {
    static INSTALL: Once = Once::new();
    INSTALL.call_once(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("ring should be the first rustls crypto provider installed");
    });
}
