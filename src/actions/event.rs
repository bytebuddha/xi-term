use crossterm::event::{ Event, KeyModifiers, KeyEvent, KeyCode };

pub fn parse_event(raw_input: &str) -> Option<Event> {
    let input = raw_input.to_lowercase();
    let mut modifiers = KeyModifiers::empty();
    let mut code = None;

    let strings = input.split(' ');

    for string in strings {
        match string {
            "control" | "ctrl" => {
                if !modifiers.contains(KeyModifiers::CONTROL) {
                    modifiers.insert(KeyModifiers::CONTROL);
                }
                continue;
            },
            "shift" | "shft" => {
                if !modifiers.contains(KeyModifiers::SHIFT) {
                    modifiers.insert(KeyModifiers::SHIFT);
                }
                continue;
            }
            "alt" => {
                if !modifiers.contains(KeyModifiers::ALT) {
                    modifiers.insert(KeyModifiers::ALT);
                }
                continue;
            },
            other => {
                if &other[0..1] == "f" {
                    if let Ok(num) = other[1..].parse() {
                        code = Some(KeyCode::F(num));
                    }
                    continue;
                } else {
                    if let Some(chr) = other.chars().next() {
                        code = Some(KeyCode::Char(chr));
                    }
                }
            }
        }
    }
    code.map(|code| Event::Key(KeyEvent { code, modifiers }))
}
