mod router;
pub use router::*;
mod renderer;
mod server;

pub struct App(String, u16);

impl App {
    pub fn new(host: &str, port: u16) -> Self {
        Self(host.to_string(), port)
    }

    pub fn start(&self, router: FrostRouter) {
        server::FrostSever::new(&self.0, self.1).start(router);
    }
}
