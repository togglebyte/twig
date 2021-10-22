use std::time::Duration;
use std::collections::VecDeque;

use anathema::{Pos, Size, Window, Cursor, Input, ScrollBuffer, Line, Lines, Instruction};
use tinylog::{Filter, LogClient, LogEntry, Saved, init_logger, Request};

fn entry_to_lines(entry: &LogEntry<Saved>, width: usize) -> Vec<Line> {
    let mut lines = Lines::new(width);
    lines.push_str(&entry.timestamp.format("%H:%M:%S ").to_string());
    lines.push_str(&entry.message);
    lines.complete()
}

#[tokio::main]
async fn main() {
    init_logger().await.unwrap();

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
    log_win.enable_scroll();

    let mut log_client = LogClient::connect_tcp("127.0.0.1:5566").await; 
    log_client.send(Request::Tail(100, None)).unwrap();
    log_client.send(Request::Subscribe(None)).unwrap();

    let mut entries = VecDeque::with_capacity(1024);

    let mut scroll_buffer = ScrollBuffer::<Line>::new(window.size().height as usize, 1024);

    loop {
        // 1. Get log events
        while let Ok(entry) = log_client.try_recv() {
            if entries.len() == entries.capacity() {
                entries.pop_front();
            }

            entry_to_lines(&entry, log_win.size().width as usize)
                .into_iter()
                .for_each(|l| scroll_buffer.push(l));

            entries.push_back(entry);
        }

        // 2. Read keyboard input / resize event
        while let Some(input) = window.get_input() {
            match input {
                Input::KeyResize => {
                    let size = window.size();
                    // Clear the scroll buffer
                    // Rebuild the buffer
                    // scroll_buffer.clear();
                    scroll_buffer.resize(size.height as usize);
                }
                Input::Character('j') => {
                    scroll_buffer.scroll_down(1);
                }
                Input::Character('k') => {
                    scroll_buffer.scroll_up(1);
                }
                Input::Character('\t') => {
                    log::info!("TAB!");
                }
                _ => {}
            }
        }

        // 3. Draw everythign
        status_win.erase();
        status_win.draw_box();
        status_win.refresh();

        input_win.erase();
        input_win.draw_box();
        input_win.refresh();

        // Fix this (somehow)
        log_win.draw_box();
        for line in scroll_buffer.lines() {
            for instruction in line.instructions() {
                match instruction {
                    Instruction::String(s) => {
                        log_win.print(s);
                    }
                    _ => {}
                }
            }

            if line.width() < log_win.size().width as usize {
                log_win.add_char('\n');
            }
        }
        log_win.refresh();

        // 4. Sleepy time
        window.nap(Duration::from_millis(100));
    }
}
