use std::time::Duration;

use tinybit::appstate::{
    AppState, Event, Events, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, Run, Sender, UserModel,
    WaitFor,
};
use tinybit::attributes::Value;
use tinybit::screen::{Output, OutputConfig};
use tinybit::widgets::{Border, RootWidget, Text, Viewport, Widget};
use tinybit::{term_size, Color};

mod input;
mod input_widget;
mod log;

type EventSender = Sender<Event<log::LogEntry>>;

pub struct LogModel {
    log: log::Log,
    event_tx: EventSender,
    input: input::Input,
}

impl LogModel {
    pub fn new(event_tx: EventSender) -> Self {
        Self { log: log::Log::new(), event_tx, input: input::Input::new() }
    }

    fn toggle_input_mode(&mut self, root: &mut RootWidget) {}
}

impl UserModel for LogModel {
    type Message = log::LogEntry;

    fn event(&mut self, event: Event<Self::Message>, root: &mut RootWidget) {
        match event {
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollUp, .. }) => {
                let output = root.by_name_mut("output").unwrap().to::<Viewport>();
                output.scroll_back(1);
            }
            Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollDown, .. }) => {
                let output = root.by_name_mut("output").unwrap().to::<Viewport>();
                output.scroll_forward(1);
            }
            Event::User(entry) => self.log.new_entry(entry),
            Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }) => {
                drop(self.event_tx.send(Event::Quit));
            }
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) if self.input.enabled() => {
                let row = root.by_name_mut("input-row").unwrap();
                self.input.toggle(row);
            }
            // Input input
            Event::Key(KeyEvent { code, modifiers }) if self.input.enabled() => {
                let output_id = root.by_name_mut("output").unwrap().id();
                let row = root.by_name_mut("input-row").unwrap();
                if let Some(s) = self.input.event(code, modifiers, row) {
                    // self.log.new_entry(s); // ignore this for now

                    let mut border = Border::default();
                    border.width = Some(usize::MAX);

                    border
                        .builder()
                        .set_name("the name")
                        .add_child(Text::with_text(s).builder())
                        .attach(output_id, root)
                        .unwrap();

                    // let output_id = root.by_name_mut("output").unwrap().id();
                    // let template = format!("border [width: max]:\n    text: '{}'", s);
                    // root.add_template(output_id, &template);

                    // output.set_text(s);
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Tab, .. }) if !self.input.enabled() => {
                let row = root.by_name_mut("input-row").unwrap();
                self.input.toggle(row);
            }
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    tinylog::init_logger(true).unwrap();

    // -----------------------------------------------------------------------------
    //     - Template setup -
    // -----------------------------------------------------------------------------
    let template = std::fs::read_to_string("template.tiny")?;
    let size = term_size();
    let mut root = RootWidget::new(size);
    root.lookup.register("input", &input_widget::input_widget);
    root.add_template(root.id(), &template)?;

    // -----------------------------------------------------------------------------
    //     - Events -
    // -----------------------------------------------------------------------------
    let events = Events::unbounded();

    // -----------------------------------------------------------------------------
    //     - User model -
    // -----------------------------------------------------------------------------
    let model = LogModel::new(events.tx());

    // -----------------------------------------------------------------------------
    //     - Output -
    // -----------------------------------------------------------------------------
    let output_config = OutputConfig { enable_mouse: true, raw_mode: true };
    let output = Output::stdout(output_config)?;

    // -----------------------------------------------------------------------------
    //     - App state -
    // -----------------------------------------------------------------------------
    let mut app = AppState::new(model, events, root, output, WaitFor::Timeout(Duration::from_millis(20)))?;

    while let Ok(Run::Continue) = app.wait_for() {}

    Ok(())
}
