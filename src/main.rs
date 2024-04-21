use std::{ fmt::Write, time::Duration, thread };

mod res;

pub fn main() {
    let mut screen = res::screen::DebugScreen::new();

    writeln!(screen, "Welcome to Para Wodna Sync for PlayStation Vita!").ok();

    thread::sleep(Duration::from_secs(5));
}
