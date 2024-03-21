use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Signal {
    Command(Command),
    Event(Event),
}

impl Signal {
    pub fn handle(&self) -> Vec<Signal> {
        match self {
            Signal::Command(Command::Start) => {
                println!("Starting");
                vec![Signal::Event(Event::Started)]
            }
            Signal::Command(Command::Stop) => {
                println!("Stopping");
                vec![Signal::Event(Event::Stopped)]
            }
            Signal::Event(Event::Started) => {
                println!("Started");
                vec![]
            }
            Signal::Event(Event::Stopped) => {
                println!("Stopped");
                vec![]
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    Start,
    Stop,
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    Started,
    Stopped,
}
