use xi_term::widgets::view::Chunk;
use tui::style::Color;
use tui::buffer::Buffer;
use tui::backend::TestBackend;
use tui::terminal::Terminal;

#[test]
fn simple_plain() {
    let backend = TestBackend::new(9, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|mut f| {
        let chunk = Chunk::new("Some line");
        f.render_widget(chunk, f.size());
    }).unwrap();

    let expected = Buffer::with_lines(vec![
        "Some line"
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn simple_plain_has_foreground() {
    let backend = TestBackend::new(9, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|mut f| {
        let chunk = Chunk::new("Some line").foreground(Some(Color::Cyan));
        f.render_widget(chunk, f.size());
    }).unwrap();

    let mut expected = Buffer::with_lines(vec![
        "Some line"
    ]);
    for line in 0..9 {
        expected.get_mut(line, 0).set_fg(Color::Cyan);
    }

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn simple_plain_has_background() {
    let backend = TestBackend::new(9, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|mut f| {
        let chunk = Chunk::new("Some line").background(Some(Color::Cyan));
        f.render_widget(chunk, f.size());
    }).unwrap();

    let mut expected = Buffer::with_lines(vec![
        "Some line"
    ]);
    for line in 0..9 {
        expected.get_mut(line, 0).set_bg(Color::Cyan);
    }

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn simple_contains_tab() {
    let backend = TestBackend::new(12, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|mut f| {
        let chunk = Chunk::new("Some\tline");
        f.render_widget(chunk, f.size());
    }).unwrap();

    let expected = Buffer::with_lines(vec![
        "Some    line"
    ]);

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn simple_contains_tab_has_foreground() {
    let backend = TestBackend::new(12, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|mut f| {
        let chunk = Chunk::new("Some\tline").foreground(Some(Color::Cyan));
        f.render_widget(chunk, f.size());
    }).unwrap();

    let mut expected = Buffer::with_lines(vec![
        "Some    line"
    ]);
    for line in 0..12 {
        expected.get_mut(line, 0).set_fg(Color::Cyan);
    }

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn simple_contains_tab_has_background() {
    let backend = TestBackend::new(12, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|mut f| {
        let chunk = Chunk::new("Some\tline").background(Some(Color::Cyan));
        f.render_widget(chunk, f.size());
    }).unwrap();

    let mut expected = Buffer::with_lines(vec![
        "Some    line"
    ]);
    for line in 0..12 {
        expected.get_mut(line, 0).set_bg(Color::Cyan);
    }

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn simple_plain_contains_newline() {
    let backend = TestBackend::new(10, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|mut f| {
        let chunk = Chunk::new("Some\nline");
        f.render_widget(chunk, f.size());
    }).unwrap();

    let expected = Buffer::with_lines(vec![
        "Some^Jline"
    ]);

    terminal.backend().assert_buffer(&expected);
}
