use rockies::Game;

fn main() -> () {
    let mut game = Game::new(64, 64);
    /*  for x in 2..62 {
        for y in 2..62 {
            game.click(x, y);
        }
    } */

    loop {
        game.key_down(' ');
        for _ in 0..100 {
            game.tick();
            game.stats();
            print!("{}",game.text_render());
        }
        game.key_up(' ');
        game.key_down('d');
        game.key_down('k');
        for _ in 0..10 {
            game.tick();
            game.stats();
            print!("{}",game.text_render());
        }
        game.key_up('d');
        game.key_up('k');
    }
}
