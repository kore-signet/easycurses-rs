[![Crates](https://img.shields.io/crates/v/easycurses.svg)](https://crates.io/crates/easycurses)

# EasyCurses

A rust crate to smooth over the pain points of working with curses. Because it's
based on [pancurses](https://github.com/ihalila/pancurses), it works equally
well with on both windows and unix computers.

Here's a basic demo:

```rust
extern crate easycurses;

use easycurses::*;

fn main() {
    let mut easy = EasyCurses::initialize_system();
    easy.set_cursor_visibility(CursorVisibility::Invisible);
    easy.set_echo(false);
    easy.print("Hello world.");
    easy.refresh();
    easy.get_input();
}
```

Unfortunately when you've got curses active rust's normal panic printing doesn't
end up working right. The panic message prints before curses does cleanup, and
then it's erased by the cleanup faster than you can read it.

I've got you covered with a wrapper function that does the `catch_unwind` for
you:

```rust
extern crate easycurses;

use easycurses::*;

fn main() {
    preserve_panic_message(|easy| {
        easy.set_cursor_visibility(CursorVisibility::Invisible);
        easy.set_echo(false);
        easy.print("Hello world.");
        easy.refresh();
        easy.get_input();
        panic!("oh no");
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("Error Occurred: {}", errmsg),
        None => println!("There was an error, but no error message."),
    });
}
```

It is currently suggested in the _strongest possible terms_ that you not attempt
to initialize curses while it's already active, but this isn't actually
enforced. In the future I might make this into a harder error if there's a way
to make it not disruptive to the normal library user (who is assumed to be well
behaved). There's _probably_ no way to do it statically, so it will just have to
be a runtime panic the moment you try to double-initialize.

Similarly, if you ever abort the program the cleanup guarantee goes right out
the window, since it's based on `Drop` working properly. So, just don't ever
abort the program.

## Stability

I would characterize the library as largely stable. It's missing at least one
feature that I'd like to see added (insert_char) but that's because pancurses
itself lacks that at the moment.

As laid out somewhere during the [high-quality
rust](https://github.com/rust-lang/rust-roadmap/issues/9) roadmap goal
discusssions, a crate can't rightly call itself 1.0 unless all the things it
depends on are themselves 1.0, so no matter what this crate won't actaully go to
1.0 before `pancurses` does.

## License

This project uses Rust's standard Apache/MIT dual-license scheme. In other
words, you can use it under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.txt) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT.txt) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
