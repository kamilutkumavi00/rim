use rim::{screen::Screen, window::Window, app::App};

fn motions((_x, _y, _x_s, _y_s): (u16, u16, u16, u16)) -> (u16, u16, u16, u16){
    (0,0,0,0)
}
fn visuals(s: (u16,u16), size:(u16,u16), mut data:Vec<String>) -> Vec<String>{
    let mut data_virtual: Vec<String> = Vec::new();
    for i in s.1..s.1+size.1{
        if i < data.len() as u16{
            if data[i as usize].len() >= s.0 as usize+size.0 as usize{
                data_virtual.push(data[i as usize].clone()[s.0 as usize..s.0 as usize+size.0 as usize].to_string());
            } else {
                data_virtual.push(data[i as usize].clone()[s.0 as usize..data[i as usize].len()].to_string());
            }
        }
    }
    data_virtual
}
fn main() {
    let mut vector: Vec<String> = Vec::new();
    vector.push(String::from("Kamil U"));
    vector.push(String::from("Kamil Utku"));
    vector.push(String::from("Kamil Utku"));
    vector.push(String::from("Kamil Utku"));
    let dv = visuals((0,0), (10,5), vector);
    let mut vector: Vec<String> = Vec::new();
    vector.push(String::from("1"));
    vector.push(String::from("2"));
    vector.push(String::from("3"));
    vector.push(String::from("4"));
    let dv2 = visuals((0,0), (1,5), vector);

    let mut screen = Screen::new();

    let mut text_editor = Window::new("Text Editor".to_string(), Vec::new(), (80,0), (50,8));
    //let a = App::new((4,1), (10,5),dv ,(0,0,0,0), motions);
    //let b = App::new((1,1), (1,5),dv2 ,(0,0,0,0), motions);
    //text_editor = text_editor.add(a);
    //text_editor = text_editor.add(b);
    screen = screen.add(text_editor);


    let mut vector: Vec<String> = Vec::new();
    vector.push(String::from("1"));
    vector.push(String::from("2"));
    vector.push(String::from("3"));
    vector.push(String::from("4"));
    vector.push(String::from("5"));
    let dv2 = visuals((0,0), (1,5), vector);
    let mut file_explorer = Window::new("File Explorer".to_string(), Vec::new(), (0,0), (10,10));
    //let b = App::new((1,1), (1,5),dv2 ,(0,0,0,0), motions);
    //file_explorer = file_explorer.add(b);
    screen = screen.add(file_explorer);

    screen = screen.update();
    screen.render();
}
