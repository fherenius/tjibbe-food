mod backend;
mod frontend;
mod shared;

#[cfg(feature = "tui")]
fn main() {
    frontend::tui::run().unwrap();
}
