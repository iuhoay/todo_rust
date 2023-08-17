mod menu;
mod todo;

use menu::Menu;

fn main() {
    Menu::new().render();
}
