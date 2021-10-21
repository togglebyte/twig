use std::time::Duration;

use anathema::{Pos, Size, Window, Cursor};
use tinylog::Filter;

#[tokio::main]
async fn main() {
    let window = Window::main(true).expect("Can't initialise the main window");
    window.no_delay(true);
    window.set_cursor_visibility(Cursor::Hide);

    let size = window.size();

    let status_size = Size::new(size.width, 3);
    let status_win = window.new_window(Pos::zero(), status_size).unwrap();

    let input_size = Size::new(size.width, 3);
    let input_win = window.new_window(
        Pos::new(0, size.height - input_size.height),
        Size::new(size.width, 3)
    ).unwrap();

    let log_win = window.new_window(
        Pos::new(0, status_size.height),
        Size::new(size.width, size.height - (input_size.height + status_size.height))
    ).unwrap();

    loop {
        // 1. Get log events

        // 2. Read keyboard input / resize event
        while let Some(input) = window.get_input() {}

        // 3. Draw everythign
        status_win.erase();
        status_win.draw_box();
        status_win.refresh();

        input_win.erase();
        input_win.draw_box();
        input_win.refresh();

        log_win.erase();
        log_win.draw_box();
        log_win.refresh();

        // 4. Sleepy time
        window.nap(Duration::from_millis(100));
    }
}
