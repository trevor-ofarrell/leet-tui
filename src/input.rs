use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn key_to_bytes(key: &KeyEvent) -> Option<Vec<u8>> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let alt = key.modifiers.contains(KeyModifiers::ALT);
    let shift = key.modifiers.contains(KeyModifiers::SHIFT);

    match key.code {
        KeyCode::Char(c) => {
            if ctrl {
                // Ctrl+key combinations
                match c {
                    'a'..='z' => Some(vec![(c as u8) - b'a' + 1]),
                    '@' => Some(vec![0]),
                    '[' => Some(vec![27]),
                    '\\' => Some(vec![28]),
                    ']' => Some(vec![29]),
                    '^' => Some(vec![30]),
                    '_' => Some(vec![31]),
                    _ => None,
                }
            } else if alt {
                // Alt+key combinations send ESC followed by the key
                Some(format!("\x1b{}", c).into_bytes())
            } else {
                Some(c.to_string().into_bytes())
            }
        }
        KeyCode::Enter => Some(b"\r".to_vec()),
        KeyCode::Backspace => {
            if ctrl {
                Some(b"\x17".to_vec()) // Ctrl+W
            } else {
                Some(b"\x7f".to_vec())
            }
        }
        KeyCode::Delete => Some(b"\x1b[3~".to_vec()),
        KeyCode::Tab => {
            if shift {
                Some(b"\x1b[Z".to_vec())
            } else {
                Some(b"\t".to_vec())
            }
        }
        KeyCode::Esc => Some(b"\x1b".to_vec()),
        KeyCode::Up => {
            if alt {
                Some(b"\x1b\x1b[A".to_vec())
            } else if ctrl {
                Some(b"\x1b[1;5A".to_vec())
            } else {
                Some(b"\x1b[A".to_vec())
            }
        }
        KeyCode::Down => {
            if alt {
                Some(b"\x1b\x1b[B".to_vec())
            } else if ctrl {
                Some(b"\x1b[1;5B".to_vec())
            } else {
                Some(b"\x1b[B".to_vec())
            }
        }
        KeyCode::Left => {
            if alt {
                Some(b"\x1b\x1b[D".to_vec())
            } else if ctrl {
                Some(b"\x1b[1;5D".to_vec())
            } else {
                Some(b"\x1b[D".to_vec())
            }
        }
        KeyCode::Right => {
            if alt {
                Some(b"\x1b\x1b[C".to_vec())
            } else if ctrl {
                Some(b"\x1b[1;5C".to_vec())
            } else {
                Some(b"\x1b[C".to_vec())
            }
        }
        KeyCode::Home => Some(b"\x1b[H".to_vec()),
        KeyCode::End => Some(b"\x1b[F".to_vec()),
        KeyCode::PageUp => Some(b"\x1b[5~".to_vec()),
        KeyCode::PageDown => Some(b"\x1b[6~".to_vec()),
        KeyCode::Insert => Some(b"\x1b[2~".to_vec()),
        KeyCode::F(n) => {
            match n {
                1 => Some(b"\x1bOP".to_vec()),
                2 => Some(b"\x1bOQ".to_vec()),
                3 => Some(b"\x1bOR".to_vec()),
                4 => Some(b"\x1bOS".to_vec()),
                5 => Some(b"\x1b[15~".to_vec()),
                6 => Some(b"\x1b[17~".to_vec()),
                7 => Some(b"\x1b[18~".to_vec()),
                8 => Some(b"\x1b[19~".to_vec()),
                9 => Some(b"\x1b[20~".to_vec()),
                10 => Some(b"\x1b[21~".to_vec()),
                11 => Some(b"\x1b[23~".to_vec()),
                12 => Some(b"\x1b[24~".to_vec()),
                _ => None,
            }
        }
        _ => None,
    }
}
