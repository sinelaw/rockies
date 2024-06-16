use rockies::Universe;

fn main() -> () {
    let mut uni = Universe::new(16, 16);
    for x in 2..14 {
        for y in 2..14 {
            uni.click(x, y);
        }
    }

    for _ in 0..10000 {
        uni.tick();
        //println!("{}\n\n", uni.text_render());
        //std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
