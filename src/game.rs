use std::thread;

pub fn wait_for_game() {
    thread::sleep(std::time::Duration::from_secs(5));
}
