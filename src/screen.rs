mod window;
use window::Window;
pub struct Screen {
    height: u16,
    width: u16,
    pub interface: String,
    window_vec: Vec<Window>,
}

impl Screen {
    pub fn new() -> Self {
        let (width, height) = (214,15);//terminal::size().unwrap();
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

    pub fn add(mut self, name: String, start: (u16, u16), size: (u16, u16)) -> Self{
        let window = Window::new(name, Vec::new(), start, size);
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
        let mut chr_vec: Vec<char>= Vec::new();
        let mut idx_vec: Vec<u16> = Vec::new();

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
                            chr_vec.push(a);
                            idx_vec.push(j + (i) * (self.width + 2));
                        }
                    }
                }
            }
        }
        let interface = replacer_total(self.interface, chr_vec, idx_vec);
        println!("{}", interface);
    }
}

fn replacer_total(text: String, chr_vec: Vec<char>, idx_vec: Vec<u16>) -> String{
    let mut text_output = String::new();
    let mut c = 0;
    for(i, j) in text.chars().enumerate(){
        if c < idx_vec.len(){
            if idx_vec[c] != i as u16{
                text_output.push(j);
            } else {
                text_output.push(chr_vec[c]);
                c += 1;
            }
        }  else{
            text_output.push(j);
        }       
    }
    text_output
}