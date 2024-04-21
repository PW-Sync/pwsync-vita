use std::fmt::Write;

mod res;

pub fn main() {
    let mut screen = res::screen::DebugScreen::new();
    writeln!(screen, "Welcome to Para Wodna Sync for PlayStation Vita!").ok();
}
