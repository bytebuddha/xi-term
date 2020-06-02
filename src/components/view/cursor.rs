use tui::layout::Rect;

use super::View;
use core::RenderCursor;

impl RenderCursor for View {

    fn render_cursor(&self, area: Rect) -> Option<(u16, u16)> {
        info!("rendering cursor");
        if self.cache.is_empty() {
            trace!("cache is empty, rendering cursor at the top left corner");
            return None;
        }

        if (self.cursor.line) < self.cache.before() {
            warn!(
                "the cursor is on line {} render_cursor which is marked invalid in the cache",
                self.cursor.line
            );
            return None;
        }
        // Get the line that has the cursor
        //if self.cursor.line < area.y as u64 {
        //    warn!("the cursor line {} is not within the visible rect {:?}", self.cursor.line, area);
        //    return None;
        //}
        let line_idx = (self.cursor.line) - self.cache.before();
        let line = match self.cache.lines().get(line_idx as usize) {
            Some(line) => line,
            None => {
                warn!("no valid line at cursor index {}", self.cursor.line);
                return None;
            }
        };

        if line_idx < (self.window.start()) {
            warn!(
                "the line that has the cursor (nb={}, cache_idx={}) not within the displayed window ({:?})",
                self.cursor.line + area.y as u64,
                line_idx,
                self.window
            );
            return None;
        }
        // Get the line vertical offset so that we know where to draw it.
        let line_pos = line_idx - self.window.start() + area.y as u64;

        // Calculate the cursor position on the linerender_cursor. The trick is that we know the position within
        // the string, but characters may have various lengths. For the moment, we only handle
        // control characters and tabs. We assume control characters (0x00-0x1f, excluding 0x09 ==
        // tab) are rendered in caret notation and are thus two columns wide. Tabs are
        // variable-width, rounding up to the next tab stop. All other characters are assumed to be
        // one column wide.
        let column: u16 = line
            .text
            .chars()
            .take(self.cursor.column as usize)
            .fold(0, |acc, c| acc + self.translate_char_width(acc, c));
        Some((column as u16 + area.x + 1, line_pos as u16))
    }
}
