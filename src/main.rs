use iced::{
  button, scrollable, text_input, Button, Column, Container, Element, Length, Radio, Row, Sandbox,
  Scrollable, Settings, Text, TextInput,
};

mod style;

pub fn main() -> iced::Result {
  App::run(Settings {
    antialiasing: true,
    ..Settings::default()
  })
}

#[derive(Default)]
struct App {
  theme: style::Theme,
  scroll: scrollable::State,
  input: text_input::State,
  input_value: String,
  button: button::State,
  code_blocks: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
  ThemeChanged(style::Theme),
  InputChanged(String),
  ButtonPressed,
}

impl Sandbox for App {
  type Message = Message;

  fn new() -> Self {
    App::default()
  }

  fn title(&self) -> String {
    String::from("Styling - Iced")
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::ThemeChanged(theme) => self.theme = theme,
      Message::InputChanged(value) => self.input_value = value,

      Message::ButtonPressed if !self.input_value.is_empty() => {
        self.code_blocks.push(self.input_value.clone());
        self.input_value.clear();
      }
      Message::ButtonPressed => {}
    }
  }

  fn view(&mut self) -> Element<Message> {
    let choose_theme = style::Theme::ALL.iter().fold(
      Column::new().spacing(10).push(Text::new("Choose a theme:")),
      |column, theme| {
        column.push(
          Radio::new(
            *theme,
            format!("{:?}", theme),
            Some(self.theme),
            Message::ThemeChanged,
          )
          .style(self.theme),
        )
      },
    );

    let text_input = TextInput::new(
      &mut self.input,
      "Put your equation here...",
      &self.input_value,
      Message::InputChanged,
    )
    .padding(10)
    .size(20)
    .style(self.theme);

    let button = Button::new(&mut self.button, Text::new("Add"))
      .padding(10)
      .on_press(Message::ButtonPressed)
      .style(self.theme);

    let sidebar = Scrollable::new(&mut self.scroll)
      .spacing(20)
      .padding(20)
      .style(self.theme)
      .push(choose_theme)
      .push(Row::new().spacing(10).push(text_input).push(button));

    let canvas = Row::new()
      .padding(20)
      .push(Text::new(format!("{:?}", self.code_blocks)));

    let content = Row::new()
      .push(canvas.width(Length::FillPortion(70)).height(Length::Fill))
      .push(sidebar.width(Length::FillPortion(30)).height(Length::Fill));

    Container::new(content).style(self.theme).into()
  }
}
