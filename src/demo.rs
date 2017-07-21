
#![allow(non_snake_case)]

// This attribute thingy requires rust 1.18 or later to work
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate easycurses;

use easycurses::*;

fn main() {
    unwind_safe_curses(|easy|{
        easy.set_cursor_visibility(CursorVisibility::Invisible);
        easy.noecho();
        easy.print("hello.");
        easy.get_char();
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("Error Occurred: {}",errmsg),
        None => println!("There was an error, but no error message."),
    });
}
