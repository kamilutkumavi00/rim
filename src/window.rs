//use crate::app::App;
#[derive(Clone, Debug)]
pub struct Window{
    name: String,
    lines: Vec<String>,
    pub start: (u16, u16),
    pub size: (u16, u16),
    pub visual_line: Vec<String>,

}

fn window_framer(size: (u16, u16), visual_line: Vec<String>) -> Vec<String>{ // there is going to have a hash map for boxes
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
                let a = if j <= visual_line.len() as u16{
                    visual_line[j as usize - 1].chars().nth(i as usize - 1).unwrap_or(' ')
                } else { 
                    ' ' 
                };
                temp[j as usize].push(a);
            }
        }
    }
    temp
}

impl Window{
    pub fn new(name: String, lines: Vec<String>, start: (u16, u16), size: (u16, u16)) -> Self{
        Self { name: name, lines: lines, start: start, size: size, visual_line: Vec::new()}
    }

    pub fn update(self) -> Self{ //motionlar kontrol edilecek ve visual appten çekilecek
        let temp: Vec<String> = window_framer(self.size, self.visual_line);
        Self { name: self.name, lines: self.lines, start: self.start, size: self.size, visual_line: temp}
    }

    //pub fn motions() -> Self {}

}