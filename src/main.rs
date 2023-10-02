use rim::screen::Screen;
fn main() {
    let mut screen = Screen::new();
    screen = screen.add("utku".to_string(), (50,0), (26,8));
    screen = screen.add("kamil".to_string(), (10,0), (26,10));
    screen = screen.update();
    screen.render();
}
