#[derive(Clone, Debug)]
pub struct Window{
    name: String,
    lines: Vec<String>,
    pub start: (u16, u16),
    pub size: (u16, u16),
    pub visual_line: Vec<String>,
}


impl Window{
    pub fn new(name: String, lines: Vec<String>, start: (u16, u16), size: (u16, u16)) -> Self{
        let mut temp: Vec<String> = Vec::new();
        temp.push(String::from("Merhabalar Arkadaşlar"));
        temp.push(String::from("Benim Adım Kamil Utku Mavi"));
        temp.push(String::from("Benim Adım Kamil Utku Mavi"));
        temp.push(String::from("Benim Adım Kamil Utku Mavi"));

        Self { name: name, lines: lines, start: start, size: size, visual_line: temp}
    }

    //pub fn add(){}

    pub fn update(self) -> Self{
        Self { name: self.name, lines: self.lines, start: self.start, size: self.size, visual_line: self.visual_line}
    }

    //pub fn render(){}
}