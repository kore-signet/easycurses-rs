
#![warn(missing_docs)]
#![allow(dead_code)]

//! This is a crate that allows one to easily use a basic form of curses. It is
//! based upon [pancurses](https://docs.rs/crate/pancurses/0.8.0) and so it's
//! cross platform between windows and unix. It exposes a simplified view of
//! curses functionality where there's just one Window and all of your actions
//! are called upon a single struct type, `EasyCurses`. This ensures that curses
//! functions are only called while curses is initialized, and also that curses
//! is always cleaned up at the end (via `Drop`).
//!
//! You should _never_ make a second `EasyCurses` value without having ensured
//! that the first one is already dropped. Initialization and shutdown of curses
//! will get out of balance and your terminal will probably be left in a very
//! unusable state.

extern crate pancurses;

use std::panic::*;

/// The three options you can pass to `EasyCurses::set_cursor_visibility`. Note
/// that not all terminals support all visibility modes.
pub enum CursorVisibility {
    /// Makes the cursor invisible.
    Invisible,
    /// Makes the cursor visible in the normal way.
    Visible,
    /// Makes the cursor "highly" visible in some way.
    HighlyVisible,
}

/// The curses color constants.
///
/// Curses supports eight colors, and each cell has one "color pair" set which
/// is a foreground and background pairing. In some implementations you can
/// change the RGB values associated with a color, and when you do that affects
/// all cells in the screen using that color in either foreground or background.
/// Note also that a cell can possibly be either bold/bright, normal, or dim, so
/// you potentially have a few more colors to work with there too.
///
/// Even if you _can_ change the color content of a color, you still access the
/// eight colors with these names.
pub enum Color {
    /// Black
    Black,
    /// Red
    Red,
    /// Green
    Green,
    /// Yellow
    Yellow,
    /// Blue
    Blue,
    /// Magenta
    Magenta,
    /// Cyan
    Cyan,
    /// White
    White,
}

/// Converts a `Color` to the `i16` associated with it.
fn color_to_i16(color: Color) -> i16 {
    use Color::*;
    match color {
        Black => 0,
        Red => 1,
        Green => 2,
        Yellow => 3,
        Blue => 4,
        Magenta => 5,
        Cyan => 6,
        White => 7,
    }
}

/// Converts an `i16` to the `Color` associated with it. Fails if the input is
/// outside the range 0 to 7 (inclusive).
fn i16_to_color(val: i16) -> Option<Color> {
    use Color::*;
    match val {
        0 => Some(Black),
        1 => Some(Red),
        2 => Some(Green),
        3 => Some(Yellow),
        4 => Some(Blue),
        5 => Some(Magenta),
        6 => Some(Cyan),
        7 => Some(White),
        _ => None,
    }
}

/// A color pair for a character cell on the screen.
pub struct ColorPair(i16);

impl ColorPair {
    /// Converts a foreground and background color into the ColorPair to use.
    pub fn from(foreground: Color, background: Color) -> Self {
        let fgi = color_to_i16(foreground);
        let bgi = color_to_i16(background);
        ColorPair(ColorPair::fgbg_pairid(fgi, bgi))
    }

    /// The "low level" conversion using i16 values. Color pair 0 is white on
    /// black but we can't assign to it. Technically we're only assured to have
    /// color pairs 0 through 63 available, but you usually get more so we're
    /// taking a gamble that there's at least one additional bit available. The
    /// alternative is a somewhat complicated conversion scheme where we special
    /// case White/Black to be 0, then other things start ascending above that,
    /// until we hit where White/Black should be and start subtracting one from
    /// everything to keep it within spec. I don't wanna do that if I don't
    /// really have to.
    fn fgbg_pairid(fg: i16, bg: i16) -> i16 {
        1 + (8 * fg + bg)
    }
}

/// Converts a `pancurses::OK` value into `true`, and all other values into
/// `false`.
fn to_bool(curses_bool: i32) -> bool {
    if curses_bool == pancurses::OK {
        true
    } else {
        false
    }
}

/// Wraps the use of curses with `catch_unwind` to preserve panic info.
///
/// Normally, if your program panics while in curses mode the panic message
/// prints immediately and then is destroyed before you can see it by the
/// automatic cleanup of curses mode. Instead, this runs the function you pass
/// it within `catch_unwind` and when there's an error it attempts to downcast
/// the result into a message you can print out or log or whatever you like.
/// Regardless of what kind of `Result` you get back, curses mode is fully
/// cleaned up and shut down by the time this function returns.
///
/// Note that you *don't* have to use this if you just want your terminal
/// restored to normal when your progam panics while in curses mode. That is
/// handled automatically by the `Drop` implementation of `EasyCurses`. You only
/// need to use this if you care about the panic message itself.
pub fn preserve_panic_message<F: FnOnce(&mut EasyCurses) -> R + UnwindSafe, R>(
    user_function: F,
) -> Result<R, Option<String>> {
    let result = catch_unwind(|| {
        let mut easy = EasyCurses::initialize_system();
        user_function(&mut easy)
    });
    result.map_err(|e| match e.downcast_ref::<&str>() {
        Some(andstr) => Some(andstr.to_string()),
        None => {
            match e.downcast_ref::<String>() {
                Some(string) => Some(string.to_string()),
                None => None,
            }
        }
    })
}

/// This is a handle to all your fun curses functionality.
///
/// `EasyCurses` will automatically restore the terminal when you drop it, so
/// you don't need to worry about any manual cleanup. Automatic cleanup will
/// happen even if your program panics and unwinds, but it **will not** happen
/// if your program panics and aborts (obviously). So, don't abort the program
/// while curses is active, or your terminal session will just be ruined.
pub struct EasyCurses {
    win: pancurses::Window,
    color_support: bool,
}

impl Drop for EasyCurses {
    /// Dropping EasyCurses causes
    /// [endwin](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/endwin.html)
    /// to be called.
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl EasyCurses {
    /// Initializes the curses system so that you can begin using curses. Note
    /// that since this uses
    /// [initscr](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/initscr.html),
    /// any error during initialization will generally cause the program to
    /// print an error message to stdout and then immediately exit. C libs are
    /// silly like that.
    ///
    /// If the terminal supports colors, they are automatcially activated and
    /// color pairs are initialized for all color foreground and background
    /// combinations.
    pub fn initialize_system() -> Self {
        let w = pancurses::initscr();
        let color_support = if pancurses::has_colors() {
            to_bool(pancurses::start_color())
        } else {
            false
        };
        if color_support {
            for fg in 0..8 {
                for bg in 0..8 {
                    pancurses::init_pair(ColorPair::fgbg_pairid(fg, bg), fg, bg);
                }
            }
        }
        EasyCurses {
            win: w,
            color_support: color_support,
        }
    }

    /// Attempts to assign a new cursor visibility. If this is successful you
    /// get a `Some` back with the old setting inside. If this fails you get a
    /// `None` back. For more info see
    /// [curs_set](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/curs_set.html)
    pub fn set_cursor_visibility(&mut self, vis: CursorVisibility) -> Option<CursorVisibility> {
        use CursorVisibility::*;
        let result = pancurses::curs_set(match vis {
            Invisible => 0,
            Visible => 1,
            HighlyVisible => 2,
        });
        match result {
            0 => Some(Invisible),
            1 => Some(Visible),
            2 => Some(HighlyVisible),
            _ => None,
        }
    }

    /// In character break mode (cbreak), characters typed by the user are made
    /// available immediately, and erase/kill/backspace character processing is
    /// not performed. When this mode is off (nocbreak) user input is not
    /// available to the application until a newline has been typed. The initial
    /// mode is not specified (but happens to often be cbreak). The bool result
    /// indicates if the operation was successful or not.
    ///
    /// See also the [Input
    /// Mode](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/intov.html#tag_001_005_002)
    /// section of the curses documentation.
    pub fn set_character_break(&mut self, cbreak: bool) -> bool {
        if cbreak {
            to_bool(pancurses::cbreak())
        } else {
            to_bool(pancurses::nocbreak())
        }
    }

    /// Disables input echoing. There is currently no way to re-enable it later
    /// because `pancurses` doesn't implement
    /// [echo](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/echo.html).
    pub fn noecho(&mut self) {
        pancurses::noecho();
    }

    /// Checks if the current terminal supports the use of colors.
    pub fn is_color_terminal(&mut self) -> bool {
        self.color_support
    }

    /// Sets the current color pair of the window. Output at any location will
    /// use this pair until a new pair is set.
    pub fn set_color_pair(&mut self, pair: ColorPair) {
        self.win.color_set(pair.0);
    }

    /// Prints the given string into the window. The bool indicates if the
    /// operation was successful or not.
    pub fn print(&mut self, string: &str) -> bool {
        to_bool(self.win.printw(string))
    }

    /// Plays an audible beep if possible, if not the screen is flashed. If
    /// neither is available then nothing happens.
    pub fn beep(&mut self) {
        pancurses::beep();
    }

    /// Flashes the screen if possible, if not an audible beep is played. If
    /// neither is available then nothing happens.
    pub fn flash(&mut self) {
        pancurses::flash();
    }

    /// Gets a character from the curses input buffer.
    pub fn get_char(&mut self) -> Option<pancurses::Input> {
        self.win.getch()
    }

    /// Discards all type-ahead that has been input by the user but not yet read
    /// by the program.
    pub fn flush_input(&mut self) {
        pancurses::flushinp();
    }
}
