use once_cell::sync::Lazy;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const HOME_PAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const LICENSE: &'static str = env!("CARGO_PKG_LICENSE");
pub const REPO_URL: &'static str = env!("CARGO_PKG_REPOSITORY");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const NAME: &'static str = "Dust-Mail";

pub struct Protocol {
    name: String,
}

impl Protocol {
    pub fn new<N: Into<String>>(name: N) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

pub const INCOMING_PROTOCOLS: Lazy<Vec<Protocol>> = Lazy::new(|| vec![Protocol::new("IMAP")]);

pub const OUTGOING_PROTOCOLS: Lazy<Vec<Protocol>> = Lazy::new(|| vec![Protocol::new("SMTP")]);
