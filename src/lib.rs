
#![warn(missing_docs)]

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

/// This is an enum for the three options you can pass to
/// `EasyCurses::set_cursor_visibility`. Note that not all terminals support all
/// visibility modes.
pub enum CursorVisibility {
    /// Makes the cursor invisible.
    Invisible,
    /// Makes the cursor visible in the normal way.
    Visible,
    /// Makes the cursor "highly" visible in some way.
    HighlyVisible,
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
    pub fn initialize_system() -> Self {
        let w = pancurses::initscr();
        EasyCurses { win: w }
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

    /// Disables input echoing. There is currently no way to re-enable it later
    /// because `pancurses` doesn't implement
    /// [echo](http://pubs.opengroup.org/onlinepubs/7908799/xcurses/echo.html).
    pub fn noecho(&mut self) {
        pancurses::noecho();
    }

    /// Prints the given string into the window. The bool indicates if the
    /// operation was successful or not.
    pub fn print(&mut self, string: &str) -> bool {
        to_bool(self.win.printw(string))
    }

    /// Gets a character from the curses input buffer.
    pub fn get_char(&mut self) -> Option<pancurses::Input> {
        self.win.getch()
    }
}
