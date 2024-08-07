use std::{
    io::{stdin, stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

mod console;

use fnv::{FnvHashMap, FnvHashSet};
use rockies::Game;

fn main() -> () {
    let mut game = Game::new(64, 64);

    let mut stdin_handle = stdin();
    let termios = console::prepare_stdin(&stdin_handle);

    let mut out = stdout();
    let mut last_frame_time = Instant::now();
    let mut last_tick_time = Instant::now();

    let mut keys: FnvHashSet<char> = FnvHashSet::default();
    
    loop {
        let frames_ms = 100;
        let tick_ms = 10;

        let start = Instant::now();
        let since_last_tick = start.duration_since(last_tick_time).as_millis();
        // throttle ticks
        if tick_ms > since_last_tick {
            sleep(Duration::from_millis((tick_ms - since_last_tick) as u64));
            last_tick_time = start;
        }
        game.tick();

        // draw frames / interact only if enough time passed
        if frames_ms < start.duration_since(last_frame_time).as_millis() {
            console::clear(&mut out);
            out.write_all(game.text_render().as_bytes()).unwrap();
            last_frame_time = start;
        }

        let mut next_keys: FnvHashSet<char> = FnvHashSet::default();
        let key = console::get_key(&mut stdin_handle);
        match key {
            Option::Some(c) => {
                next_keys.insert(c);
                match c {
                    'a'..='z' | 'A'..='Z' | ' ' => {
                        if !keys.contains(&c) {
                        game.key_down(c);
                        }
                    }
                    // CTRL+C
                    '\u{3}' => break,
                    _ => (), //println!("key: {:?}", key),
                }
            }
            None => (),
        }
        for k in keys {
            if !next_keys.contains(&k) {
                game.key_up(k);
            }
        }
        keys = next_keys;
    }

    console::unprepare_stdin(&stdin_handle, termios);

    /*
    loop {
        game.key_down(' ');
        for _ in 0..100 {
            game.tick();
            game.stats();
            print!("{}", game.text_render());
        }
        game.key_up(' ');
        game.key_down('d');
        game.key_down('k');
        for _ in 0..10 {
            game.tick();
            game.stats();
            print!("{}", game.text_render());
        }
        game.key_up('d');
        game.key_up('k');
    } */
}
