use iced::{Application, Program, Task};

pub fn main() -> iced::Result {
    application().run()
}

fn application() -> Application<impl Program<Message = Message>> {
    iced::application(Huginn::new, Huginn::update, Huginn::view)
        .title("Runar Huginn")
        .centered()
}

#[derive(Debug)]
enum Huginn {
    WelcomeScreen,
}

impl Huginn {
    fn new() -> Self {
        Self::WelcomeScreen
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            Self::WelcomeScreen => match message {},
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        match self {
            Self::WelcomeScreen => iced::widget::container(
                iced::widget::text("Hello, world!").size(32),
            )
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .into(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {}
