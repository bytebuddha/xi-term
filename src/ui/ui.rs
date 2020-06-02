use std::io::{self, Stdout, stdout};

use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::sync::oneshot::{self, Receiver, Sender};
use futures::{Async, Future, Poll, Sink, Stream};

use tui::backend::CrosstermBackend;
use tui::terminal::Terminal as TuiTerminal;
use xrl::{Client, Frontend, MeasureWidth, XiNotification};

use failure::Error;

use core::{ EventHandler, RenderCursor };
use actions::ActionReactor;
use ui::{Terminal, TerminalEvent};
use components::{ Editor, Prompt };
use widgets::{ PromptWidget, EditorWidget };

pub struct XiTerm {
    /// The editor holds the text buffers (named "views" in xi
    /// terminology).
    pub editor: Editor,

    /// The terminal is used to draw on the screen a get inputs from
    /// the user.
    pub terminal: Terminal,
    pub term: TuiTerminal<CrosstermBackend<Stdout>>,
    pub current_size: Option<(u16, u16)>,

    pub actions: ActionReactor,
    pub prompt: Option<Prompt>,

    /// Whether the editor is shutting down.
    pub exit: bool,

    /// Stream of messages from Xi core.
    pub core_events: UnboundedReceiver<CoreEvent>,
}

impl XiTerm {
    /// Create a new Tui instance.
    pub fn new(client: Client, events: UnboundedReceiver<CoreEvent>) -> Result<Self, Error> {
        Ok(XiTerm {
            terminal: Terminal::new()?,
            term: TuiTerminal::new(CrosstermBackend::new(stdout()))?,
            exit: false,
            editor: Editor::new(client),
            prompt: None,
            core_events: events,
            actions: ActionReactor::default(),
            current_size: None
        })
    }

    pub fn handle_resize(&mut self, size: (u16, u16)) {
        self.current_size = Some(size);
        self.editor.handle_resize(size);
    }

    fn render(&mut self) -> Result<(), Error> {

        let XiTerm { term, editor, prompt, .. } = self;

        if let Some(prompt) = &prompt {
            let mut rect = None;
            term.draw(|mut f| {
                let prompt_rect = f.size();
                let editor_widget = EditorWidget::new();
                f.render_stateful_widget(editor_widget, prompt_rect, editor);
                let prompt = PromptWidget::new(&prompt);
                f.render_widget(prompt, prompt_rect);
                rect = Some(prompt_rect);
            })?;
            if let Some(size) = rect {
                let column: u16 = prompt.chars
                    .chars()
                    .take(prompt.dex)
                    .fold(0, |acc, c| acc + translate_char_width(acc, c));
                term.set_cursor(column + 2, size.height)?;
            }
        } else {
            let mut rect = None;
            term.draw(|mut f| {
                let editor_rect = f.size();
                let editor_widget = EditorWidget::new();
                f.render_stateful_widget(editor_widget, editor_rect, editor);
                rect = Some(EditorWidget::calculate_view_rect(editor.display_title_bar, editor.display_gutter, editor_rect));
            })?;
            if let Some(size) = rect {
                if let Some(view) = editor.views.get(&editor.current_view) {
                    if let Some(cursor) = view.render_cursor(size) {
                        term.set_cursor(cursor.0, cursor.1)?;
                    }
                }
            }
        }

        Ok(())
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
                    TerminalEvent::Input(event) => self.handle_event(event),
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
                Ok(Async::Ready(Some(event))) => self.editor.handle_core_event(event),
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

fn translate_char_width(position: u16, c: char) -> u16 {
    match c {
        // Caret notation means non-tab control characters are two columns wide
        '\x00'..='\x08' | '\x0a'..='\x1f' | '\x7f' => 2,
        '\t' => 4 - (position % 4),
        _ => 1,
    }
}
