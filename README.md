# EasyCurses

A rust crate to smooth over the pain points of working with curses.

```rust
extern crate easycurses;

use easycurses::*;

fn main() {
    let mut easy = EasyCurses::initialize_system();
    easy.print("Hello world.");
    easy.set_cursor_visibility(CursorVisibility::Invisible);
    easy.set_echo(false);
    easy.get_input();
}
```

Unfortunately when you've got curses active rust's normal panic printing doesn't
end up working right. The panic message prints before curses does cleanup, and
then it's erased by the cleanup faster than you can read it. I've got you
covered there too.

```rust
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
```

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
