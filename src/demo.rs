
#![allow(non_snake_case)]

// This attribute requires rust 1.18 or later to work
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate easycurses;

use easycurses::*;

fn main() {
    preserve_panic_message(|easy| {
        easy.set_cursor_visibility(CursorVisibility::Invisible);
        easy.noecho();
        easy.print("hello.\n");
        //easy.set_character_break(false);
        let c = easy.get_char();
        easy.print(&format!("{:?}\n", c));
        easy.get_char();
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("Error Occurred: {}", errmsg),
        None => println!("There was an error, but no error message."),
    });
}
