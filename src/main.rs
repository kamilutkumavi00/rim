use rim::screen::Screen;
fn main() {
    let mut screen = Screen::new();
    screen = screen.add("name".to_string(), (0,0), (26,4));
    screen = screen.update();
    screen.render();
    //println!("{}", screen.interface);
}
