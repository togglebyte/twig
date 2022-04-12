use unicode_width::UnicodeWidthChar;
use tinybit::appstate::{KeyCode, KeyModifiers};
use tinybit::widgets::{RootWidget, Text, WidgetContainer, Display, Position};
use tinybit::attributes::Value;
use tinybit::Color;

struct Cursor {
    byte_index: usize,
    pos: i32,
}

impl Cursor {
    fn new() -> Self {
        Self {
            byte_index: 0,
            pos: 0,
        }
    }
    fn move_left(&mut self, c: char) {
        self.pos -= c.width().unwrap_or(0) as i32;
        self.byte_index -= c.len_utf8();
    }

    fn move_right(&mut self, c: char) {
        self.pos += c.width().unwrap_or(0) as i32;
        self.byte_index += c.len_utf8();
    }
}

pub struct Input {
    input_enabled: bool,
    text: String,
    cursor: Cursor,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input_enabled: false,
            text: String::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn enabled(&self) -> bool {
        self.input_enabled
    }

    pub fn event(&mut self, code: KeyCode, modifiers: KeyModifiers, widget: &mut WidgetContainer) -> Option<String> {
        // if modifiers == KeyModifiers::CONTROL {
        //     match code {
        //         KeyCode::Char('u') => {} // delete to previous word
        //     }
        // }

        match code {
            KeyCode::Enter => {
                self.cursor = Cursor::new();
                let text = widget.by_name_mut("input").unwrap().to::<Text>();
                text.set_text("");
                let cursor = widget.by_name_mut("cursor").unwrap().to::<Position>();
                cursor.left(self.cursor.pos);
                return Some(self.text.drain(..).collect());
            }
            KeyCode::Char(c) => {
                self.text.push(c);
                self.cursor.move_right(c);
            }
            KeyCode::Backspace => {
                if self.cursor.byte_index == self.text.len() {
                    if let Some(c) = self.text.pop() {
                        self.cursor.move_left(c);
                    }
                } else {
                    let c = self.text.remove(self.cursor.byte_index);
                    self.cursor.move_left(c);
                }
            }
            KeyCode::Delete => {
                if self.cursor.byte_index < self.text.len() {
                    let c = self.text.remove(self.cursor.byte_index);
                }
            }
            KeyCode::Left if self.cursor.pos > 0 => {
                let border_title = widget.by_name_mut("border-title").unwrap().to::<Text>();

                for (bi, c) in self.text.char_indices() {
                    if bi + c.len_utf8() == self.cursor.byte_index {
                        border_title.set_text(format!("[{c}] > "));
                        self.cursor.move_left(c);
                        break;
                    }
                }
            }
            KeyCode::Right if self.cursor.byte_index < self.text.len() => {
                let border_title = widget.by_name_mut("border-title").unwrap().to::<Text>();

                for (bi, c) in self.text.char_indices() {
                    if bi == self.cursor.byte_index {
                        let i = bi+c.len_utf8();
                        if let Some(c) = self.text.get(i..).and_then(|s| s.chars().next()) {
                            border_title.set_text(format!("[{c}] > "));
                        } else {
                            border_title.set_text(format!("[ ] > "));
                        }
                        self.cursor.move_right(c);
                        break;
                    }
                }
            }
            _ => {}
        }

        let text = widget.by_name_mut("input").unwrap().to::<Text>();
        text.set_text(&self.text);

        let cursor = widget.by_name_mut("cursor").unwrap().to::<Position>();
        cursor.left(self.cursor.pos);

        None
    }

    pub fn toggle(&mut self, row: &mut WidgetContainer) {
        self.input_enabled = !self.input_enabled;

        match self.input_enabled {
            true => {
                // row.set_attribute("background", Color::Red);
                row.by_name_mut("cursor").map(|c| c.display = Display::Show);
            }
            false => {
                // row.set_attribute("background", Color::Reset);
                row.by_name_mut("cursor").map(|c| c.display = Display::Exclude);
            }
                
        }
    }
}
