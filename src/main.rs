use rockies::Universe;

fn main() -> () {
    let mut uni = Universe::new(16, 16);
    uni.click(2, 2);
    for _ in 0..1000000 {
        uni.tick();
        //println!("{}\n\n", uni.text_render());
        //std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
