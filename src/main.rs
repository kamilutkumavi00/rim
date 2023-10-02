use rim::screen::Screen;
fn main() {
    let mut screen = Screen::new();
    screen = screen.add("utku".to_string(), (50,0), (55,8));
    screen = screen.add("kamil".to_string(), (0,0), (26,10));
    screen = screen.update();
    let mut yazi: Vec<String> = Vec::new();
    yazi.push("Selam Arkadaşlar nasılsınız bende iyiyim inşallah".to_string());
    yazi.push("Benim adım".to_string());
    yazi.push("Kamil Utku".to_string());
    yazi.push("Mavi".to_string());
    screen.render();
}
