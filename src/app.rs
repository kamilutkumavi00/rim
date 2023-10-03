#[derive(Clone, Debug)]
pub struct App{
    start: (u16, u16),
    size: (u16, u16),
    data: Vec<String>,
    visual_data: Vec<String>,
    loc: (u16, u16, u16, u16),
    motion_fn: fn(()),
}

impl App{
    fn new(){

    }

    fn update(){

    }
}