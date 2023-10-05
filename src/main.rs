use rim::{screen::Screen, window::Window, app::App};
fn motions((_x, _y, _x_s, _y_s): (u16, u16, u16, u16)) -> (u16, u16, u16, u16){
    (0,0,0,0)
}
fn visuals(s: (u16,u16), size:(u16,u16), data:Vec<String>) -> Vec<String>{
    let mut data_virtual: Vec<String> = Vec::new();
    for i in s.1..s.1+size.1{
        if i < data.len() as u16{
            data_virtual.push(data[i as usize].clone()[s.0 as usize..s.0 as usize+size.0 as usize].to_string());
        }
    }
    data_virtual
}
fn main() {
    let mut vector: Vec<String> = Vec::new();
    vector.push(String::from("Kamil Utku"));
    vector.push(String::from("Kamil Utku"));
    vector.push(String::from("Kamil Utku"));
    vector.push(String::from("Kamil Utku"));
    let dv = visuals((4,0), (5,5), vector);
    let mut screen = Screen::new();

    let mut text_editor = Window::new("Text Editor".to_string(), Vec::new(), (50,0), (50,8));
    text_editor.visual_line = dv;
    let a = App::new((0,0), (1,1), (0,0,0,0), motions);
    screen = screen.add(text_editor);


    let file_explorer = Window::new("File Explorer".to_string(), Vec::new(), (0,0), (10,10));
    screen = screen.add(file_explorer);

    screen = screen.update();
    screen.render();
}
