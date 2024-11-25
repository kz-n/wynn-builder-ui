#![allow(unused)]
use std::process::Stdio;

use futures::{SinkExt, Stream};
use iced::{
    alignment::{Horizontal, Vertical},
    stream::try_channel,
    task, Element, Length, Task,
};
use iced_widget::{button, column, container, scrollable, text, Container};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    select,
};

use crate::config::style::WARNING;
use crate::{BuilderMessage, Message};

#[derive(Default)]
pub struct Builder {
    state: State,
}

#[derive(Default, Debug, Clone)]
pub enum BuilderProgress {
    Running(String),
    Done,
    Error(String),
    #[default]
    None,
}

impl Builder {
    pub fn update(&mut self, message: BuilderMessage) -> Task<Message> {
        match message {
            BuilderMessage::Communication(line) => {
                let progress = if let Ok(progress) = line {
                    progress
                } else {
                    BuilderProgress::Error(line.err().unwrap())
                };

                let new_content = match progress {
                    BuilderProgress::Running(str) => str,
                    BuilderProgress::Done => {
                        self.state.is_running = false;
                        "finished running builder binary".to_string()
                    }
                    BuilderProgress::Error(str) => {
                        self.state.is_running = false;
                        str
                    }
                    BuilderProgress::None => "".to_string(),
                };

                if self.state.text.len() >= 100 {
                    self.state.text.remove(0);
                }

                self.state.text.push(new_content);

                Task::none()
            }
            BuilderMessage::StartBinary => {
                let (state, task) = State::new();
                self.state = state;

                task
            }
            BuilderMessage::StopBinary => {
                self.state._process.abort();
                self.state.is_running = false;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Container<Message> {
        let column = column![
        text("Builder").size(30),
        text("This tab is where the builder binary is run and monitored.").size(20),
        text("Beware: Running the builder binary with the output builds flag enabled will generate a lot of output and may lag or even crash the application.")
        .size(20)
        .color(WARNING),
        if self.state.is_running {
            Element::new(
                button("Stop Builder")
                    .padding(10)
                    .on_press(Message::Builder(BuilderMessage::StopBinary)),
            )
        } else {
            Element::new(
                button("Start Builder")
                    .padding(10)
                    .on_press(Message::Builder(BuilderMessage::StartBinary)),
            )
        },
            scrollable(column(self.state.text.iter().map(|s| text(s).into())))
                .width(Length::Fill)
        ]
        .padding(10)
        .spacing(10);

        container(column)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Top)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct State {
    is_running: bool,
    text: Vec<String>,
    _process: task::Handle,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_running: false,
            text: vec![],
            _process: {
                let (_, handle) = Task::<Result<String, String>>::none().abortable();
                handle
            },
        }
    }
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let (task, handle) = Task::run(start_binary(), |result| {
            Message::Builder(BuilderMessage::Communication(result))
        })
        .abortable();

        let instance = Self {
            is_running: true,
            _process: handle.abort_on_drop(),
            ..Default::default()
        };

        (instance, task)
    }
}

pub fn start_binary() -> impl Stream<Item = Result<BuilderProgress, String>> {
    let binary_name = if cfg!(target_os = "windows") {
        "builder.exe"
    } else {
        "builder"
    };

    try_channel(1, move |mut output| async move {
        let mut _process = tokio::process::Command::new(binary_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| format!("Failed to start binary: {}", e))?;

        let stdout = _process.stdout.take().unwrap();

        let mut output_buffer = BufReader::new(stdout).lines();

        loop {
            let output_future = output_buffer.next_line();

            select! {
                output_result = output_future => {
                    let Ok(result) = output_result else {
                        return Err(format!("Failed to read stdout"));
                    };

                    let Some(line) = result else {
                        return Err(format!("Failed to read stdout: reached end of stream"));
                    };

                    if line.contains("done") {
                        let _ = output.send(BuilderProgress::Running(line)).await;
                        let _ = output.send(BuilderProgress::Done).await;
                        break Ok(());
                    }

                    let _ = output.send(BuilderProgress::Running(line)).await;
                },
            }
        }
    })
}
