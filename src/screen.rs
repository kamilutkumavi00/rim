use crate::window::Window;
use std::collections::HashMap;
pub struct Screen {
    height: u16,
    width: u16,
    pub interface: String,
    window_vec: Vec<Window>,
}

impl Screen {
    pub fn new() -> Self {
        let (width, height) = (200,10);//terminal::size().unwrap();
        let mut interface = String::new();
        for _i in 0..height {
            for _j in 0..width {
                interface.push('.');
            }
            interface.push('\r');
            interface.push('\n');
        }
        Self {
            height: height,
            width: width,
            interface: interface,
            window_vec: Vec::new(),
        }
    }

    pub fn add(mut self, window: Window) -> Self{
        //let window = Window::new(name, Vec::new(), start, size);
        self.window_vec.push(window);
        Self{
            height: self.height,
            width: self.width,
            interface: self.interface,
            window_vec: self.window_vec,
        }
        
    }

    pub fn update(self) -> Self {
        let mut temp = self.window_vec.clone();
        for i in 0..temp.len(){
            temp[i] = temp[i].clone().update();
        }
        Self { height: self.height, width: self.width, interface: self.interface, window_vec: temp }
    }

    pub fn render(self){
        let mut map: HashMap<u16, char> = HashMap::new();
        for k in 0..self.window_vec.len(){
            for i in 0..self.height {
                for j in 0..self.width {
                    if i >= self.window_vec[k].start.1 && i < self.window_vec[k].start.1 + self.window_vec[k].size.1 {
                        if j >= self.window_vec[k].start.0 && j < self.window_vec[k].start.0 + self.window_vec[k].size.0 {
                            let a = if i - self.window_vec[k].start.1 < self.window_vec[k].visual_line.len() as u16 {
                                self.window_vec[k].visual_line[(i - self.window_vec[k].start.1 ) as usize]
                                    .chars()
                                    .nth((j - self.window_vec[k].start.0) as usize)
                                    .unwrap_or(' ')
                            } else {
                                ' '
                            };
                            map.insert(j + (i) * (self.width + 2), a);
                        }
                    }
                }
            }
        }
        let interface = screen_framer(&self.interface, map);
        println!("{}", interface);
    }
}

fn screen_framer(text: &String, map: HashMap<u16, char>) -> String{
    let mut text_output = String::new();
    for(i, j) in text.chars().enumerate(){
        if map.get(&(i as u16)).is_some(){
            text_output.push(*map.get(&(i as u16)).unwrap());
        } else {
            text_output.push(j);
        }
    }
    text_output
}