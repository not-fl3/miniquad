#[derive(Debug)]
struct Modifiers {
    shift: bool,
}

pub struct XkbState {
    modifiers: Modifiers,
}

impl XkbState {
    pub const fn new() -> XkbState {
        XkbState {
            modifiers: Modifiers { shift: false },
        }
    }

    pub fn update_modifiers(&mut self, mods_depressed: u32) {
        self.modifiers.shift = (mods_depressed & 1) != 0;
    }

    pub fn keycode_to_character(&self, key: u32) -> Option<char> {
        // TODO: write long comment on how I would like to support xcb
        let us_character_map_lowercase = [
            '\0', '\0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\0', '\0',
            'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', '\0', 'a', 's', 'd',
            'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '\0', '\0', '\0', 'z', 'x', 'c', 'v', 'b',
            'n', 'm', ',', '.', '/', '\0', '\0', '\0', ' ',
        ];
        let us_character_map_uppercase = [
            '\0', '\0', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\0', '\0',
            'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n', '\0', 'A', 'S', 'D',
            'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '\0', '\0', '\0', 'Z', 'X', 'C', 'V', 'B', 'N',
            'M', '<', '>', '?', '\0', '\0', '\0', ' ',
        ];
        let active_map = if self.modifiers.shift {
            &us_character_map_uppercase
        } else {
            &us_character_map_lowercase
        };

        if key < active_map.len() as u32 {
            let character = active_map[key as usize];

            if character != '\0' {
                return Some(character);
            }
        }

        None
    }
}
