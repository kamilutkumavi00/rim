use crate::app::App;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Window{
    name: String,
    lines: Vec<String>,
    pub start: (u16, u16),
    pub size: (u16, u16),
    pub visual_line: Vec<String>,
    app_vec: Vec<App>,
}

fn window_framer(size: (u16, u16), _visual_line: Vec<String>, dict: HashMap<(u16,u16), char>) -> Vec<String>{ // there is going to have a hash map for boxes
    let mut temp: Vec<String> = Vec::new();
    for j in 0..size.1{
        temp.push(String::new());
        for i in 0..size.0{
            if j == 0 && i == 0{
                temp[j as usize].push('┏');
            } else if j == 0 && i == size.0 - 1{
                temp[j as usize].push('┓');
            } else if j == size.1 - 1 && i == 0{
                temp[j as usize].push('┗');
            } else if j == size.1 - 1 && i == size.0 - 1{
                temp[j as usize].push('┛');
            } else if j == 0 || j == size.1 - 1 {
                temp[j as usize].push('━');
            } else if i == 0 || i == size.0 - 1 {
                temp[j as usize].push('┃');
            } else {
                let a = *dict.get(&(i,j)).unwrap_or(&' ');
                temp[j as usize].push(a);
            }
        }
    }
    temp
}

impl Window{
    pub fn new(name: String, lines: Vec<String>, start: (u16, u16), size: (u16, u16)) -> Self{
        Self { name: name, lines: lines, start: start, size: size, visual_line: Vec::new(), app_vec: Vec::new()}
    }

    pub fn add(mut self, app: App) -> Self{
        self.app_vec.push(app);
        Self { name: self.name, lines: self.lines, start: self.start, size: self.size, visual_line: self.visual_line, app_vec: self.app_vec}
    }

    pub fn update(self) -> Self { //motionlar kontrol edilecek ve visual appten çekilecek
        let a = make_dict(&self.app_vec, self.size);
        let temp: Vec<String> = window_framer(self.size, self.visual_line, a);
        Self { name: self.name, lines: self.lines, start: self.start, size: self.size, visual_line: temp, app_vec: self.app_vec}
    }

    //pub fn motions() -> Self {}
}

fn make_dict(app: &Vec<App>, size: (u16, u16)) -> HashMap<(u16, u16), char>{
    let mut temp: HashMap<(u16,u16), char> = HashMap::new();
    for i in app{
        for j in 0..size.1{
            for k in 0..size.0{
                if (j >= i.start.1 && j < i.start.1 + i.size.1) && (k >= i.start.0 && k < i.start.0 + i.size.0) && j - i.start.1 < i.visual_data.len() as u16 {
                    if k - i.start.0 < i.visual_data[j as usize - i.start.1 as usize].len() as u16{
                        let v = i.visual_data[j as usize - i.start.1 as usize].chars().nth(k as usize - i.start.0 as usize).unwrap();
                        temp.insert((k,j), v);

                    } else {
                        if temp.get(&(k,j)) == None {
                            temp.insert((k,j), ' ');
                        } else {
    
                        }
                    }
                } else {
                    if temp.get(&(k,j)) == None {
                        temp.insert((k,j), ' ');
                    } else {

                    }
                }
            }
        }
    }
    temp
}
