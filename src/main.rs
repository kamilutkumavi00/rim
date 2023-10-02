use rim::{screen::Screen, window::Window};
fn main() {
    let mut screen = Screen::new();
    let text_editor = Window::new("Text Editor".to_string(), Vec::new(), (50,0), (50,8));
    let file_explorer = Window::new("File Explorer".to_string(), Vec::new(), (0,0), (10,10));
    screen = screen.add(text_editor);
    screen = screen.add(file_explorer);
    screen = screen.update();
    // let mut yazi: Vec<String> = Vec::new();
    // yazi.push("Selam Arkadaşlar nasılsınız bende iyiyim inşallah".to_string());
    // yazi.push("Benim adım".to_string());
    // yazi.push("Kamil Utku".to_string());
    // yazi.push("Mavi".to_string());
    screen.render();
}
