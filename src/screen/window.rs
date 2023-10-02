#[derive(Clone, Debug)]
pub struct Window{
    name: String,
    lines: Vec<String>,
    pub start: (u16, u16),
    pub size: (u16, u16),
    pub visual_line: Vec<String>,
    loc: (u16, u16, u16, u16), //(x, y, x_s, y_s)
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
        let mut yazi: Vec<String> = Vec::new();
        let mut numara: Vec<String> = Vec::new();
        numara.push("1".to_string());
        numara.push("2".to_string());
        numara.push("3".to_string());
        numara.push("4".to_string());
        yazi.push("Selam Arkadaşlar nasılsınız bende iyiyim inşallah".to_string());
        yazi.push("Benim adım".to_string());
        yazi.push("Kamil Utku".to_string());
        yazi.push("Mavi".to_string());

        let temp: Vec<String> = window_framer(size, yazi);
        Self { name: name, lines: lines, start: start, size: size, visual_line: temp, loc:(0,0,0,0)}
    }

    //pub fn add(){}

    pub fn update(self) -> Self{
        Self { name: self.name, lines: self.lines, start: self.start, size: self.size, visual_line: self.visual_line, loc: self.loc}
    }

    //pub fn motions() -> Self {}

}