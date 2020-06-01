use std::io::{self, Stdout, stdout};

use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::sync::oneshot::{self, Receiver, Sender};
use futures::{Async, Future, Poll, Sink, Stream};

use tui::backend::CrosstermBackend;
use tui::terminal::Terminal as TuiTerminal;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use xrl::{Client, Frontend, MeasureWidth, XiNotification};

use failure::Error;

use core::{Terminal, TerminalEvent};
use components::Editor;

pub struct XiTerm {
    /// The editor holds the text buffers (named "views" in xi
    /// terminology).
    pub editor: Editor,

    /// The terminal is used to draw on the screen a get inputs from
    /// the user.
    terminal: Terminal,
    term: TuiTerminal<CrosstermBackend<Stdout>>,

    /// Whether the editor is shutting down.
    exit: bool,

    /// Stream of messages from Xi core.
    core_events: UnboundedReceiver<CoreEvent>,
}

impl XiTerm {
    /// Create a new Tui instance.
    pub fn new(client: Client, events: UnboundedReceiver<CoreEvent>) -> Result<Self, Error> {
        Ok(XiTerm {
            terminal: Terminal::new()?,
            term: TuiTerminal::new(CrosstermBackend::new(stdout()))?,
            exit: false,
            editor: Editor::new(client),
            core_events: events,
        })
    }

    fn handle_resize(&mut self, size: (u16, u16)) {
        self.editor.handle_resize(size);
    }

    /// Global keybindings can be parsed here
    fn handle_input(&mut self, event: Event) {
        debug!("handling input {:?}", event);
        match event {
            Event::Key(event) => {
                if event.modifiers.contains(KeyModifiers::CONTROL) {
                    if let KeyCode::Char('c') = event.code {
                        self.exit = true
                    }
                }
                self.editor.handle_input(Event::Key(event));
            },
            event => {
                self.editor.handle_input(event);
                return;
            }
        }
    }

    fn render(&mut self) -> Result<(), Error> {

        let XiTerm { term, editor, .. } = self;
        term.draw(|mut f| {
            let editor = crate::widgets::EditorWidget::new(&editor);
            f.render_widget(editor, f.size());
        })?;
        if let Some(view) = editor.views.get(&editor.current_view) {
            let (x, y) = view.render_cursor();
            term.set_cursor(x, y)?;
        }
        // self.editor.render(self.terminal.stdout())?;
        // if let Err(e) = self.terminal.stdout().flush() {
        //     error!("failed to flush stdout: {}", e);
        // }
        Ok(())
    }

    fn handle_core_event(&mut self, event: CoreEvent) {
        self.editor.handle_core_event(event)
    }

    fn poll_editor(&mut self) {
        debug!("polling the editor");
        match self.editor.poll() {
            Ok(Async::NotReady) => {
                debug!("no more editor event, done polling");
                return;
            }
            Ok(Async::Ready(_)) => {
                info!("The editor exited normally. Shutting down the TUI");
                self.exit = true;
                return;
            }
            Err(e) => {
                error!("The editor exited with an error: {:?}", e);
                error!("Shutting down the TUI.");
                self.exit = true;
                return;
            }
        }
    }

    fn poll_terminal(&mut self) {
        debug!("polling the terminal");
        loop {
            match self.terminal.poll() {
                Ok(Async::Ready(Some(event))) => match event {
                    TerminalEvent::Input(event) => self.handle_input(event),
                    TerminalEvent::Resize(event) => self.handle_resize(event),
                },
                Ok(Async::Ready(None)) => {
                    info!("The terminal exited normally. Shutting down the TUI");
                    self.exit = true;
                    return;
                }
                Ok(Async::NotReady) => {
                    debug!("no more terminal event, done polling");
                    return;
                }
                Err(e) => {
                    error!("The terminal exited with an error: {:?}", e);
                    error!("Shutting down the TUI");
                    self.exit = true;
                    return;
                }
            }
        }
    }

    fn poll_rpc(&mut self) {
        debug!("polling for RPC messages");
        loop {
            match self.core_events.poll() {
                Ok(Async::Ready(Some(event))) => self.handle_core_event(event),
                Ok(Async::Ready(None)) => {
                    info!("The RPC endpoint exited normally. Shutting down the TUI");
                    self.exit = true;
                    return;
                }
                Ok(Async::NotReady) => {
                    debug!("no more RPC event, done polling");
                    return;
                }
                Err(e) => {
                    error!("The RPC endpoint exited with an error: {:?}", e);
                    error!("Shutting down the TUI");
                    self.exit = true;
                    return;
                }
            }
        }
    }
}

impl Future for XiTerm {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        debug!("polling the TUI");
        self.poll_terminal();
        if self.exit {
            info!("exiting the TUI");
            return Ok(Async::Ready(()));
        }

        self.poll_editor();
        if self.exit {
            info!("exiting the TUI");
            return Ok(Async::Ready(()));
        }

        self.poll_rpc();
        if self.exit {
            info!("exiting the TUI");
            return Ok(Async::Ready(()));
        }

        debug!("done polling the TUI components");
        debug!("rendering");
        self.render().expect("failed to render the TUI");
        debug!("done rendering, end of polling");
        Ok(Async::NotReady)
    }
}

pub enum CoreEvent {
    Notify(XiNotification),
    MeasureWidth((MeasureWidth, Sender<Vec<Vec<f32>>>)),
}

pub struct XiTermService(pub UnboundedSender<CoreEvent>);

impl Frontend for XiTermService {
    type NotificationResult = Result<(), ()>;
    fn handle_notification(&mut self, notification: XiNotification) -> Self::NotificationResult {
        self.0.start_send(CoreEvent::Notify(notification)).unwrap();
        self.0.poll_complete().unwrap();
        Ok(())
    }

    type MeasureWidthResult = NoErrorReceiver<Vec<Vec<f32>>>;
    fn handle_measure_width(&mut self, request: MeasureWidth) -> Self::MeasureWidthResult {
        let (tx, rx) = oneshot::channel::<Vec<Vec<f32>>>();
        self.0
            .start_send(CoreEvent::MeasureWidth((request, tx)))
            .unwrap();
        self.0.poll_complete().unwrap();
        NoErrorReceiver(rx)
    }
}

/// A dummy type from wrap a `oneshot::Receiver`.
///
/// The only difference with the `oneshot::Receiver` is that
/// `NoErrorReceiver`'s future implementation uses the empty type `()`
/// for its error.
pub struct NoErrorReceiver<T>(Receiver<T>);

impl<T> Future for NoErrorReceiver<T> {
    type Item = T;
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll().map_err(|_cancelled| ())
    }
}
