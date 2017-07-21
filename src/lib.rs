
extern crate pancurses;

use std::panic::*;

/// This is an enum for the three options you can pass to
/// `set_cursor_visibility`. Note that not all terminals support all visibility
/// modes.
pub enum CursorVisibility {
    /// Makes the cursor invisible.
    Invisible,
    /// Makes the cursor visible in the normal way.
    Visible,
    /// Makes the cursor "highly" visible in some way.
    HighlyVisible,
}

fn to_bool(curses_bool: i32) -> bool {
    if curses_bool == pancurses::OK {
        true
    } else {
        false
    }
}

pub fn unwind_safe_curses<F: FnOnce(&mut EasyCurses) -> R + UnwindSafe, R>(
    user_function: F,
) -> Result<R, Option<String>> {
    let result = catch_unwind(|| {
        let mut easy = EasyCurses::new();
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

pub struct EasyCurses {
    win: pancurses::Window,
}

impl Drop for EasyCurses {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl EasyCurses {
    /// Initializes the curses system and returns a handle that lets you access
    /// EasyCurses. Note that since this uses `initscr` from curses, an error
    /// during initialization will generally cause the program to print an error
    /// message to stdout and then exit. C libs are silly like that.
    pub fn new() -> Self {
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

    /// Disables character echo. There is currently no way to enabled it later.
    pub fn noecho(&mut self) {
        pancurses::noecho();
    }

    /// Prints the given string into the window.
    pub fn print(&mut self, string: &str) -> bool {
        to_bool(self.win.printw(string))
    }

    /// Gets a character from the input stream.
    pub fn get_char(&mut self) -> Option<pancurses::Input> {
        self.win.getch()
    }
}
