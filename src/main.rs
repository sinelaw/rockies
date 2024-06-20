use rockies::Game;

fn main() -> () {
    let mut game = Game::new(64, 64);
    /*  for x in 2..62 {
        for y in 2..62 {
            game.click(x, y);
        }
    } */

    loop {
        for _ in 0..10 {
            game.tick();
            game.stats();
            game.key('w');
            game.key(' ');
            game.key('k');
        }
        println!("{}\n\n", game.text_render());
    }
}
