use futures::{channel::mpsc, stream::{self, BoxStream}, Stream};
use iced::{advanced::text, stream as iced_stream};

pub fn connect() -> impl Stream<Item = Event> {
    todo!();
    iced_stream::channel(1, |mut output| async move {
        let mut state = State::Disconnected;

        loop {
            match &mut state {
                State::Disconnected => {
                    
                },
                State::Connected(pin, receiver) => todo!(),
            }
        }
    })
}

enum State {
    Disconnected,
    Connected(BoxStream<'static, Message>, mpsc::Receiver<Message>),
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Connection),
    Disconnected,
    MessageReceived(Message),
}

#[derive(Debug, Clone)]
pub struct Connection(mpsc::Sender<Message>);

impl Connection {
    pub fn send(&mut self, message: Message) {
        self.0
            .try_send(message)
            .expect("Send message to builder failed");
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Connected,
    Disconnected,
    MessageReceived(String),
}

impl Message {
    pub fn new_message(message: String) -> Option<Self> {
        if message.is_empty() {
            None
        } else {
            Some(Self::MessageReceived(message))
        }
    }

    pub fn connected() -> Self {
        Self::Connected
    }

    pub fn disconnected() -> Self {
        Self::Disconnected
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Connected => "Connected",
            Self::Disconnected => "Disconnected",
            Self::MessageReceived(message) => message,
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> text::IntoFragment<'a> for Message {
    fn into_fragment(self) -> text::Fragment<'a> {
        text::Fragment::from(self.to_string())
    }
}
