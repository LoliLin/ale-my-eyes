use iced::{
    button, executor, text_input, Application, Button, Column, Command, Container, Element,
    Length, Settings, Text, TextInput,
};

pub fn main() -> iced::Result {
    AleApp::run(Settings::default())
}

#[derive(Default)]
struct AleApp {
    status: String,
    transcribe_button: button::State,
    synthesize_button: button::State,
    describe_button: button::State,
    text_input: text_input::State,
    text_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    TranscribePressed,
    SynthesizePressed,
    DescribePressed,
    TextChanged(String),
}

impl Application for AleApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                status: "Ready".to_string(),
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Ale, My Eyes!")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TranscribePressed => {
                self.status = "Transcribing...".to_string();
                // TODO: 实现转录功能
            }
            Message::SynthesizePressed => {
                self.status = "Synthesizing...".to_string();
                // TODO: 实现合成功能
            }
            Message::DescribePressed => {
                self.status = "Describing...".to_string();
                // TODO: 实现描述功能
            }
            Message::TextChanged(value) => {
                self.text_value = value;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Ale, My Eyes!")
            .size(30)
            .width(Length::Fill);

        let status = Text::new(&self.status)
            .size(16)
            .width(Length::Fill);

        let text_input = TextInput::new(
            &mut self.text_input,
            "Enter text to synthesize...",
            &self.text_value,
            Message::TextChanged,
        )
        .padding(10)
        .size(16);

        let transcribe_button = Button::new(
            &mut self.transcribe_button,
            Text::new("Transcribe Audio"),
        )
        .padding(10)
        .on_press(Message::TranscribePressed);

        let synthesize_button = Button::new(
            &mut self.synthesize_button,
            Text::new("Synthesize Text"),
        )
        .padding(10)
        .on_press(Message::SynthesizePressed);

        let describe_button = Button::new(
            &mut self.describe_button,
            Text::new("Describe Image"),
        )
        .padding(10)
        .on_press(Message::DescribePressed);

        let content = Column::new()
            .spacing(20)
            .push(title)
            .push(status)
            .push(text_input)
            .push(transcribe_button)
            .push(synthesize_button)
            .push(describe_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}