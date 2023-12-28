use frost::{FrostRouter, App};

fn main() {
    let router = FrostRouter::new("app").unwrap();
    App::new("localhost", 3000).start(router);
}
