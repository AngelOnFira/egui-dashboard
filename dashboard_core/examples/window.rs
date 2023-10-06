use dashboard_core::{init, ObjKind, Object};

fn main() {
    init();
    let button = Object::new(ObjKind::Button);

    // Sleep this thread indefinitely so that the server doesn't exit.
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
