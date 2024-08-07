use std::{
    io::{stdin, stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

mod console;

use fnv::FnvHashSet;
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
        let frames_ms = 30;
        let tick_ms = 30;

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

            if !process_events(&mut stdin_handle, &mut keys, &mut game) {
                break;
            }
        }
    }

    console::unprepare_stdin(&stdin_handle, termios);
}

fn process_events(
    stdin_handle: &mut std::io::Stdin,
    keys: &mut FnvHashSet<char>,
    game: &mut Game,
) -> bool {
    let mut next_keys: FnvHashSet<char> = FnvHashSet::default();
    loop {
        let key = console::get_key(stdin_handle);
        match key {
            Option::Some(c) => {
                next_keys.insert(c);
                match c {
                    // CTRL+C
                    '\u{3}' => return false,
                    _ => (), //println!("key: {:?}", key),
                }
            }
            None => break,
        }
    }
    for k in next_keys.iter() {
        if !keys.contains(k) {
            game.key_down(*k);
        }
    }
    for k in keys.iter() {
        if !next_keys.contains(k) {
            game.key_up(*k);
        }
    }
    //print!("keys: {:?}, next_keys: {:?}\n\r", keys, next_keys);
    keys.clone_from(&mut next_keys);
    true
}
