use dashboard_core::init;

fn main() {
    init();

    // Sleep this thread indefinitely so that the server doesn't exit.
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
