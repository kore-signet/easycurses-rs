
#![allow(non_snake_case)]

// This attribute requires rust 1.18 or later to work
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate easycurses;

use easycurses::*;

fn main() {
    preserve_panic_message(|easy| {
        easy.set_cursor_visibility(CursorVisibility::Invisible);
        easy.set_keypad_enabled(true);
        easy.noecho();
        easy.print("hello.\n");
        //easy.set_character_break(false);
        let c = easy.get_input();
        easy.print(&format!("{:?}\n", c));
        let (r, c) = easy.get_row_col_count();
        easy.move_xy(0, 0);
        easy.print(&format!("r:{:} c:{:}\n", r, c));
        easy.get_input();
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("Error Occurred: {}", errmsg),
        None => println!("There was an error, but no error message."),
    });
}
