
// Prevents a spare console window from being made on windows in release mode,
// but also prevents ALL console output, which this only activates in release
// mode. That way you can still see your debug messages in debug mode.
// Ironically, since Windows uses PDcurses and lets your normal console keep
// going, it's actually the best platform for easycurses development.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate easycurses;

use easycurses::*;

fn main() {
    preserve_panic_message(|easy| {
        easy.print("Hello world.");
        easy.set_cursor_visibility(CursorVisibility::Invisible);
        easy.set_echo(false);
        easy.get_input();
        panic!("oh no");
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("Error Occurred: {}", errmsg),
        None => println!("There was an error, but no error message."),
    });
}
