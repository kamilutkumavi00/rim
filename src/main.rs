use rim::{screen::Screen, window::Window, app::App};
fn main() {
    let mut screen = Screen::new();

    let text_editor = Window::new("Text Editor".to_string(), Vec::new(), (50,0), (50,8));
    screen = screen.add(text_editor);

    let file_explorer = Window::new("File Explorer".to_string(), Vec::new(), (0,0), (10,10));
    screen = screen.add(file_explorer);

    screen = screen.update();
    screen.render();
}
