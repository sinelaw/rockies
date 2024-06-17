use rockies::Universe;

fn main() -> () {
    let mut uni = Universe::new(64, 64);
    for x in 2..62 {
        for y in 2..62 {
            uni.click(x, y);
        }
    }

    for _ in 0..5000 {
        uni.tick();
        uni.stats();
        //println!("{}\n\n", uni.text_render());
        //std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
