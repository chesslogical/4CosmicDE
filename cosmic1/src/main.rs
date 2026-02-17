use iced::{
    executor,
    Application, Command, Element, Settings, Theme, Length, Size,
    widget::{button, column, container, pick_list, scrollable, text, text_input},
};
use sha2::{Digest, Sha512};
use sha3::Sha3_512;

fn main() -> iced::Result {
    ShaApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(700.0, 600.0),
            min_size: Some(Size::new(200.0, 200.0)),
            ..Default::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Sha2_512,
    Sha3_512,
}

impl Algorithm {
    const ALL: [Algorithm; 2] = [Algorithm::Sha2_512, Algorithm::Sha3_512];
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Algorithm::Sha2_512 => write!(f, "SHA-512 (SHA-2)"),
            Algorithm::Sha3_512 => write!(f, "SHA3-512"),
        }
    }
}

struct ShaApp {
    input: String,
    hashes: Vec<String>,
    selected_algorithm: Algorithm,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    GenerateHash,
    ClearHashes,
    AlgorithmSelected(Algorithm),
}

impl Application for ShaApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                input: String::new(),
                hashes: Vec::new(),
                selected_algorithm: Algorithm::Sha2_512,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SHA Hasher")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
            }

            Message::AlgorithmSelected(algo) => {
                self.selected_algorithm = algo;
            }

            Message::GenerateHash => {
                if !self.input.is_empty() {
                    let hash = match self.selected_algorithm {
                        Algorithm::Sha2_512 => {
                            let mut hasher = Sha512::new();
                            hasher.update(self.input.as_bytes());
                            format!("{:x}", hasher.finalize())
                        }
                        Algorithm::Sha3_512 => {
                            let mut hasher = Sha3_512::new();
                            hasher.update(self.input.as_bytes());
                            format!("{:x}", hasher.finalize())
                        }
                    };

                    self.hashes.push(hash);
                }
            }

            Message::ClearHashes => {
                self.hashes.clear();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let input_section = column![
            text("Enter text to hash:"),
            text_input("Type here...", &self.input)
                .on_input(Message::InputChanged)
                .width(Length::Fill),
            pick_list(
                &Algorithm::ALL[..],
                Some(self.selected_algorithm),
                Message::AlgorithmSelected,
            )
            .width(Length::Fill),
            button("Generate Hash")
                .on_press(Message::GenerateHash)
                .width(Length::Fill),
            button("Clear All")
                .on_press(Message::ClearHashes)
                .width(Length::Fill),
        ]
        .spacing(10)
        .width(Length::Fill);

        let mut hashes_column = column![].spacing(10).width(Length::Fill);

        for hash in &self.hashes {
            hashes_column = hashes_column.push(
                container(
                    text(hash)
                        .size(16)
                )
                .width(Length::Fill)
                .padding(5)
            );
        }

        let scrollable_hashes = scrollable(hashes_column)
            .width(Length::Fill)
            .height(Length::FillPortion(1));

        column![
            input_section,
            text("Generated Hashes:"),
            scrollable_hashes,
        ]
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

