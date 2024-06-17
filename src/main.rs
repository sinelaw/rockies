use rockies::Game;

fn main() -> () {
    let mut game = Game::new(64, 64);
    /*  for x in 2..62 {
        for y in 2..62 {
            game.click(x, y);
        }
    } */

    for _ in 0..5000 {
        game.tick();
        game.stats();
        println!("{}\n\n", game.text_render());
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
