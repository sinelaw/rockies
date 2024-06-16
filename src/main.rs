use rockies::Universe;

fn main() -> () {
    let mut uni = Universe::new(32, 32);
    loop {
        uni.tick();
        println!("{}\n\n", uni.text_render());
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
