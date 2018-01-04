extern crate gtk;

mod gui {
    pub mod gtk3;
}

fn main() {
    gui::gtk3::launch();
}
