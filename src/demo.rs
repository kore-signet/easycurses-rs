
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

        easy.set_bold(true);
        easy.print("hello.\n");
        easy.set_bold(false);

        let c = easy.get_input();
        easy.set_underline(true);
        easy.print(&format!("{:?}\n", c));
        easy.set_underline(false);

        let (r, c) = easy.get_row_col_count();
        easy.set_scrolling(true);
        easy.set_scroll_region(1, 5);
        easy.print(&format!("1\n2\n3\n4\n5\nr:{:} c:{:}\nend", r, c));
        easy.set_scroll_region(0, r - 1);
        easy.move_xy(0, 0);
        easy.print(&format!("finita\n"));
        easy.get_input();
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("Error Occurred: {}", errmsg),
        None => println!("There was an error, but no error message."),
    });
}
