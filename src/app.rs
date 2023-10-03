#[derive(Clone, Debug)]
pub struct App{
    start: (u16, u16),
    size: (u16, u16),
    data: Vec<String>,
    visual_data: Vec<String>,
    loc: (u16, u16, u16, u16),
    motion_fn: fn((u16, u16, u16, u16)) -> (u16, u16, u16, u16),
}

impl App{
    pub fn new(
    start: (u16, u16),
    size: (u16, u16),
    loc: (u16, u16, u16, u16),
    motion_fn: fn((u16, u16, u16, u16)) -> (u16, u16, u16, u16),
    )-> Self{
        let data = Vec::new();
        let virtual_data = Vec::new();
        Self { start: start, size: size, data: data, visual_data: virtual_data, loc: loc, motion_fn: motion_fn}
    }

    pub fn update(mut self){
        self.loc = (self.motion_fn)(self.loc);
    }
}