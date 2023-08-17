mod menu;
mod todo;

use menu::Menu;

fn main() {
    // let mut menu = menu::Menu::new();
    Menu::new().render();
}
