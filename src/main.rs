use std::{
    io::{stdin, stdout, Write},
    os::fd::AsRawFd,
    thread::sleep,
    time::{Duration, Instant},
};

mod console;

use ansi_term::{ANSIGenericString, ANSIStrings};
use fnv::FnvHashSet;
use libc::{ioctl, winsize, TIOCGWINSZ};
use rockies::Game;

static FRAMES_MS: u128 = 40;
static TICK_MS: u128 = 20;
static KBD_MS: u128 = 100;

fn main() -> () {
    let mut out = stdout();
    console::screen_save(&mut out);
    console::alternate_buffer_enable(&mut out);
    console::clear(&mut out);
    console::cursor_disable(&mut out);
    let mut stdin_handle = stdin();
    let termios = console::prepare_terimnal(&stdin_handle);

    let winsize = get_terminal_size(&out);

    let mut game = Game::new(winsize.ws_col as usize - 2, winsize.ws_row as usize - 2);

    let mut last_frame_time = Instant::now();
    let mut last_tick_time = Instant::now();
    let mut last_kbd_time = Instant::now();

    let mut keys: FnvHashSet<String> = FnvHashSet::default();

    loop {
        // throttle ticks
        let since_last_tick = last_tick_time.elapsed().as_millis();
        if TICK_MS > since_last_tick {
            sleep(Duration::from_millis((TICK_MS - since_last_tick) as u64));
        }

        let start = Instant::now();
        last_tick_time = start;
        game.tick();
        // draw frames / interact only if enough time passed
        if FRAMES_MS < start.duration_since(last_frame_time).as_millis() {
            last_frame_time = start;
            let wsize = get_terminal_size(&out);
            text_render(&mut out, &game, wsize);
        }

        // keyboard events are not really available in terminal console. We only
        // get a stream of characters from stdin to work with. If a key is being
        // held down we will get a single character, some delay, and then
        // repeats of that character. If multiple non-modifier keys are held
        // down we only get one of them as input. We don't really know if any
        // key is down or up at any given moment. As a workaround we assume that
        // any character read from stdin is a key held down for at least some
        // duration (KBD_MS) before considering that key as released.
        if KBD_MS < start.duration_since(last_kbd_time).as_millis() {
            last_kbd_time = start;
            let keep_going = process_keyboard(&mut stdin_handle, &mut keys, &mut game);
            if !keep_going {
                break;
            }
        }
    }

    console::restore_terminal(&stdin_handle, termios);
    console::cursor_enable(&mut out);
    console::alternate_buffer_disable(&mut out);
    console::screen_restore(&mut out);
}

fn get_terminal_size(out: &std::io::Stdout) -> winsize {
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let res = unsafe { ioctl(out.as_raw_fd(), TIOCGWINSZ, &mut w) };
    assert!(res == 0, "ioctl failed");
    w
}

fn process_keyboard(
    stdin_handle: &mut std::io::Stdin,
    keys: &mut FnvHashSet<String>,
    game: &mut Game,
) -> bool {
    let mut next_keys: FnvHashSet<String> = FnvHashSet::default();
    loop {
        let key = console::get_key(stdin_handle);
        match key {
            Option::Some(c) => {
                match c {
                    // CTRL+C
                    '\u{3}' => return false,
                    '\u{1}' => {
                        next_keys.insert(" ".to_string());
                        next_keys.insert("a".to_string());
                    }
                    '\u{4}' => {
                        next_keys.insert(" ".to_string());
                        next_keys.insert("d".to_string());
                    }
                    '\u{17}' => {
                        next_keys.insert(" ".to_string());
                        next_keys.insert("w".to_string());
                    }
                    '\u{13}' => {
                        next_keys.insert(" ".to_string());
                        next_keys.insert("w".to_string());
                    }
                    _ => {
                        next_keys.insert(c.to_string());
                        // println!("key: {:?}", key);
                    }
                }
            }
            None => break,
        }
    }
    // handle shift state
    let old_shift = keys.iter().any(|k| k.to_ascii_lowercase() != *k);
    let new_shift = next_keys.iter().any(|k| k.to_ascii_lowercase() != *k);
    if old_shift && !new_shift {
        game.key_up("shift".to_string());
    }
    if new_shift && !old_shift {
        game.key_down("shift".to_string());
    }

    for k in next_keys.iter() {
        if !keys.contains(k) {
            game.key_down(k.to_string());
        }
    }
    for k in keys.iter() {
        if !next_keys.contains(k) {
            game.key_up(k.to_string());
        }
    }
    //print!("keys: {:?}, next_keys: {:?}\n\r", keys, next_keys);
    keys.clone_from(&mut next_keys);
    true
}

fn text_render(out: &mut std::io::Stdout, game: &Game, wsize: winsize) -> () {
    let term_width = usize::from(wsize.ws_col);
    let padding = if term_width > game.width() {
        (term_width - game.width()) / 2
    } else {
        0
    };

    let mut frame: Vec<ANSIGenericString<str>> = Vec::default();

    for line in game.pixels_vec().chunks(game.width() as usize) {
        for _ in 0..padding {
            frame.push(" ".into());
        }
        for &pixel in line {
            let color = ansi_term::Color::RGB(
                ((pixel >> 16) & 0xFF) as u8,
                ((pixel >> 8) & 0xFF) as u8,
                (pixel & 0xFF) as u8,
            );
            let symbol = ansi_term::Style::new().fg(color).paint("█");
            frame.push(symbol);
        }
        frame.push("\n".into());
        frame.push("\r".into());
    }

    let output = ANSIStrings(frame.as_slice());

    console::home(out);
    write!(out, "{}", output).unwrap();
    out.flush().unwrap();
}
