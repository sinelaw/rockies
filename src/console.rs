use libc::cfmakeraw;
use libc::ioctl;
use libc::tcgetattr;
use libc::tcsetattr;
use libc::termios;
use libc::FIONREAD;
use libc::NCCS;
use libc::TCSANOW;
use std::io::Read;
use std::io::Stdin;
use std::io::Write;
use std::os::fd::AsRawFd;

static TERMIOS_EMPTY: libc::termios = libc::termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; NCCS],
    c_ispeed: 0,
    c_ospeed: 0,
};

pub(crate) fn prepare_stdin(stdin_handle: &Stdin) -> termios {
    let mut termios = TERMIOS_EMPTY;
    unsafe {
        let fd = stdin_handle.as_raw_fd();
        if 0 != tcgetattr(fd, &mut termios) {
            assert!(false, "tcgetattr failed");
        }
        let prev_termios = termios;
        cfmakeraw(&mut termios);
        if 0 != tcsetattr(fd, TCSANOW, &mut termios) {
            assert!(false, "tcsetattr failed");
        }
        prev_termios
    }
}

pub(crate) fn unprepare_stdin(stdin_handle: &Stdin, prev_termios: termios) {
    let mut termios = prev_termios;
    unsafe {
        let fd = stdin_handle.as_raw_fd();
        if 0 != tcsetattr(fd, TCSANOW, &mut termios) {
            assert!(false, "tcsetattr failed");
        }
    }
}

pub(crate) fn get_key(stdin_handle: &mut Stdin) -> Option<char> {
    let ready_chars = unsafe {
        let mut x: [u32; 1] = [0];
        let ioctl_res = ioctl(stdin_handle.as_raw_fd(), FIONREAD, &mut x);
        assert!(ioctl_res == 0, "ioctl failed");
        x[0]
    };
    if ready_chars == 0 {
        return None;
    }
    let mut buf: [u8; 1] = [0];
    stdin_handle.read(&mut buf).unwrap();
    Option::from(buf[0] as char)
}

pub(crate) fn clear(f: &mut impl Write) {
    write!(
        f,
        "{}{}",
        // Reset the terminal (clear the screen)
        ansi_control_codes::independent_control_functions::RIS,
        // Go to position 0,0
        ansi_control_codes::control_sequences::CUP(0.into(), 0.into())
    )
    .unwrap();
}

pub(crate) fn home(f: &mut impl Write) {
    write!(
        f,
        "{}",
        // Go to position 0,0
        ansi_control_codes::control_sequences::CUP(0.into(), 0.into())
    )
    .unwrap();
}
