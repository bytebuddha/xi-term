use futures::Future;
use tokio::spawn;
use xrl;

use crate::actions::FindAction;

pub struct Client {
    inner: xrl::Client,
    view_id: xrl::ViewId,
}

impl Client {
    pub fn new(client: xrl::Client, view_id: xrl::ViewId) -> Self {
        Client {
            inner: client,
            view_id,
        }
    }

    pub fn insert(&mut self, character: char) {
        let f = self.inner.char(self.view_id, character).map_err(|_| ());
        spawn(f);
    }

    pub fn set_lang(&mut self, lang: String) {
        let f = self.inner.set_language(self.view_id, &lang).map_err(|_| ());
        spawn(f);
    }

    pub fn insert_newline(&mut self) {
        let f = self.inner.insert_newline(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn insert_tab(&mut self) {
        let f = self.inner.insert_tab(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn scroll(&mut self, start: u64, end: u64) {
        let f = self.inner.scroll(self.view_id, start, end).map_err(|_| ());
        spawn(f);
    }

    pub fn down(&mut self) {
        let f = self.inner.down(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn up(&mut self) {
        let f = self.inner.up(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn right(&mut self) {
        let f = self.inner.right(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn left(&mut self) {
        let f = self.inner.left(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn page_down(&mut self) {
        let f = self.inner.page_down(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn page_up(&mut self) {
        let f = self.inner.page_up(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn home(&mut self) {
        let f = self.inner.line_start(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn end(&mut self) {
        let f = self.inner.line_end(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn delete(&mut self) {
        let f = self.inner.delete(self.view_id).map_err(|_| ());
        spawn(f);
    }

    pub fn backspace(&mut self) {
        let f = self.inner.backspace(self.view_id).map_err(|_| ());
        spawn(f);
    }


    pub fn save(&mut self, file: &str) {
        spawn(self.inner.save(self.view_id, file).map_err(|_| ()));
    }

    pub fn click(&mut self, line: u64, column: u64) {
        error!("Received click: {}, {}", line, column);
        let f = self
            .inner
            .click_point_select(self.view_id, line, column)
            .map_err(|_| ());
        spawn(f);
    }

    pub fn find(&mut self, action: FindAction) {
        match action {
            FindAction::Query(query, regex, case, words) => spawn(self.inner.find(self.view_id, &query, case, regex, words).map_err(|_|())),
            FindAction::Next(wrap, same) => spawn(self.inner.find_next(self.view_id, wrap, same, xrl::ModifySelection::None).map_err(|_|())),
            FindAction::Previous(wrap, same) => spawn(self.inner.find_prev(self.view_id, wrap, same, xrl::ModifySelection::None).map_err(|_|())),
        };
        spawn(self.inner.highlight_find(self.view_id, true).map_err(|_| ()));
    }
}
