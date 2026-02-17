use iced::{
    executor,
    Application, Command, Element, Settings, Theme, Length, Size,
    widget::{button, column, container, pick_list, scrollable, text, text_input, row},
};
use sha2::{Digest, Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> iced::Result {
    ShaApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(250.0, 200.0)),
            ..Default::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
    Blake3,
}

impl Algorithm {
    const ALL: [Algorithm; 5] = [
        Algorithm::Sha256,
        Algorithm::Sha512,
        Algorithm::Sha3_256,
        Algorithm::Sha3_512,
        Algorithm::Blake3,
    ];
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Algorithm::Sha256 => write!(f, "SHA-256"),
            Algorithm::Sha512 => write!(f, "SHA-512"),
            Algorithm::Sha3_256 => write!(f, "SHA3-256"),
            Algorithm::Sha3_512 => write!(f, "SHA3-512"),
            Algorithm::Blake3 => write!(f, "BLAKE3"),
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
    AppendToFile,
    Ignore,
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
                selected_algorithm: Algorithm::Sha256,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Hash Tool")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
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
                        Algorithm::Sha256 => {
                            let mut hasher = Sha256::new();
                            hasher.update(self.input.as_bytes());
                            format!("{:x}", hasher.finalize())
                        }
                        Algorithm::Sha512 => {
                            let mut hasher = Sha512::new();
                            hasher.update(self.input.as_bytes());
                            format!("{:x}", hasher.finalize())
                        }
                        Algorithm::Sha3_256 => {
                            let mut hasher = Sha3_256::new();
                            hasher.update(self.input.as_bytes());
                            format!("{:x}", hasher.finalize())
                        }
                        Algorithm::Sha3_512 => {
                            let mut hasher = Sha3_512::new();
                            hasher.update(self.input.as_bytes());
                            format!("{:x}", hasher.finalize())
                        }
                        Algorithm::Blake3 => {
                            let hash = blake3::hash(self.input.as_bytes());
                            hash.to_hex().to_string()
                        }
                    };

                    self.hashes.push(hash);
                }
            }

            Message::ClearHashes => {
                self.hashes.clear();
            }

            Message::AppendToFile => {
                if !self.hashes.is_empty() {
                    if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("hash.txt")
                    {
                        for hash in &self.hashes {
                            let _ = writeln!(file, "{}", hash);
                        }
                    }
                }
            }

            Message::Ignore => {}
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
            row![
                button("Generate Hash")
                    .on_press(Message::GenerateHash),
                button("Clear All")
                    .on_press(Message::ClearHashes),
                button("Append All to hash.txt")
                    .on_press(Message::AppendToFile),
            ]
            .spacing(10),
        ]
        .spacing(10)
        .width(Length::Fill);

        let mut hashes_column = column![].spacing(10).width(Length::Fill);

        for hash in &self.hashes {
            hashes_column = hashes_column.push(
                container(
                    text_input("", hash)
                        .on_input(|_| Message::Ignore)
                        .width(Length::Fill)
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
