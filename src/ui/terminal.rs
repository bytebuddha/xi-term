use std::io::{Write, stdout};
use std::thread::{sleep, spawn};
use std::time::Duration;

use futures::sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::{Async, Poll, Sink, Stream};

use failure::Error;

use crossterm::event::{ Event, EnableMouseCapture, DisableMouseCapture };
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::terminal::size;

pub struct Terminal {
    size: UnboundedReceiver<(u16, u16)>,
    stdin: UnboundedReceiver<Event>
}

impl Terminal {
    pub fn new() -> Result<Self, Error> {
        let (stdin_tx, stdin_rx) = unbounded();
        let (size_tx, size_rx) = unbounded();

        crossterm::execute!(stdout(), EnableMouseCapture, EnterAlternateScreen)?;

        enable_raw_mode()?;

        let term = Terminal {
            stdin: stdin_rx,
            size: size_rx
        };

        Terminal::start_stdin_listening(stdin_tx);
        Terminal::start_size_listening(size_tx);
        Ok(term)
    }

    fn start_stdin_listening(tx: UnboundedSender<Event>) {
        let mut tx = tx;
        spawn(move || {
            info!("waiting for input events");
            loop {
                if let Ok(event) = crossterm::event::read() {
                    tx.start_send(event).unwrap();
                    tx.poll_complete().unwrap();
                }
            }
        });
    }

    fn start_size_listening(tx: UnboundedSender<(u16, u16)>) {
        let mut tx = tx;
        spawn(move || {
            let mut current_size = (0, 0);
            info!("waiting for resize events");
            loop {
                match size() {
                    Ok(new_size) => {
                        if new_size != current_size {
                            info!(
                                "terminal resized (from {:?} to {:?})",
                                current_size, new_size
                            );
                            current_size = new_size;
                            let _ = tx.start_send(current_size).unwrap();
                            let _ = tx.poll_complete().unwrap();
                        }
                    }
                    Err(e) => {
                        error!("failed to get terminal size: {}", e);
                    }
                }
                sleep(Duration::from_millis(10));
            }
        });
    }
}

pub enum TerminalEvent {
    Resize((u16, u16)),
    Input(Event),
}

impl Stream for Terminal {
    type Item = TerminalEvent;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        debug!("polling for terminal size events");
        match self.size.poll() {
            Ok(Async::Ready(Some(size))) => {
                debug!("size event: {:?}", size);
                let event = TerminalEvent::Resize(size);
                return Ok(Async::Ready(Some(event)));
            }
            Ok(Async::Ready(None)) => {
                warn!("terminal size sender closed the channel");
                return Ok(Async::Ready(None));
            }
            Ok(Async::NotReady) => {
                debug!("done polling for terminal size events");
            }
            Err(()) => return Err(()),
        }

        debug!("polling for stdin events");
        match self.stdin.poll() {
            Ok(Async::Ready(Some(event))) => {
                debug!("stdin event: {:?}", event);
                let event = TerminalEvent::Input(event);
                return Ok(Async::Ready(Some(event)));
            }
            Ok(Async::Ready(None)) => {
                warn!("terminal input sender closed the channel");
                return Ok(Async::Ready(None));
            }
            Ok(Async::NotReady) => {
                debug!("done polling for stdin events");
            }
            Err(()) => return Err(()),
        }
        debug!("done polling the terminal");
        Ok(Async::NotReady)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen).unwrap();

        disable_raw_mode().unwrap();
    }
}
