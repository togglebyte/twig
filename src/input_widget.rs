use tinybit::attributes::Attributes;
use tinybit::Color;
use tinybit::widgets::{Container, Position, Row, Stack, Text, Widget, WidgetBuilder};

pub fn input_widget(attribs: &Attributes) -> Option<WidgetBuilder> {
    let row_name = attribs.name_ref();
    let input_name = attribs.get_str("input-name");
    let border_name = attribs.get_str("border-name");
    let cursor_name = attribs.get_str("cursor-name");

    let mut border_title = Text::with_text("[ ] > ");
    border_title.trim_end = false;
    let mut border_title = border_title.builder();
    if let Some(name) = border_name {
        border_title.set_name_mut(name);
    }

    let mut input = Text::new();
    input.trim_start = false;
    input.trim_end = false;
    input.collapse_spaces = false;

    let mut input = input.builder();
    if let Some(name) = input_name {
        input.set_name_mut(name);
    }

    let mut position = Position::default();
    position.left(1);
    let mut position = position.builder();
    if let Some(name) = cursor_name {
        position.set_name_mut(name);
    }

    let cursor = Container::new(1, 1);
    let mut cursor = cursor.builder();
    cursor.style.bg = Some(Color::Blue);

    let mut builder = Row::new().builder();
    if let Some(name) = row_name {
        builder.set_name_mut(name);
    }

    let builder = builder
        .add_child(Container::default()
            .builder()
            .add_child(border_title))
        .add_child(Stack::new()
            .builder()
            .add_child(position
                .add_child(cursor))
            .add_child(Container::default()
                .builder()
                .add_child(input)));

    Some(builder)
}


// row [name: "input-row"]:
//     container:
//         text [trim-end: false, name: "border-title"]: "[ ] > "
//     stack:
//         position [name: "cursor", left: 0, display: exclude]:
//             container [width: 1, height: 1, reverse: true, background: red]:
//         container:
//             text [name: "input", foreground: red, trim-start: false, collapse-spaces: false]: ""
