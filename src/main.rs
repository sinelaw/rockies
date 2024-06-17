use rockies::Universe;

fn main() -> () {
    let mut uni = Universe::new(64, 64);
    for x in 2..34 {
        for y in 2..14 {
            uni.click(x, y);
        }
    }

    for _ in 0..500 {
        uni.tick();
        uni.stats();
        //println!("{}\n\n", uni.text_render());
        //std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
