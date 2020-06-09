use xi_term::actions::parse_event;
use crossterm::event::{ Event, KeyEvent, KeyCode, KeyModifiers };

#[test]
fn parse_simple_event() {
    let input = "control shift p";

    let code = KeyCode::Char('p');
    let mut modifiers = KeyModifiers::all();
    modifiers.remove(KeyModifiers::ALT);

    assert_eq!(
        Some(Event::Key(KeyEvent { code, modifiers})),
        parse_event(input)
    );

    let input = "control alt f12";

    let code = KeyCode::F(12);
    let mut modifiers = KeyModifiers::all();
    modifiers.remove(KeyModifiers::SHIFT);

    assert_eq!(
        Some(Event::Key(KeyEvent { code, modifiers})),
        parse_event(input)
    );
}
