use std::sync::{Arc, Mutex};

use inputbot::*;

struct Bindings {
    to_bind: Vec<KeybdKey>,
    next: usize,
}

impl Bindings {
    fn new() -> Self {
        let to_bind = vec![
            KeybdKey::LControlKey,
            KeybdKey::LAltKey,
            KeybdKey::LShiftKey,
        ];

        println!("Press mouse button to bind them to the modifier key shown:");
        println!("{}: ", to_bind[0].to_string());
        Bindings { to_bind, next: 0 }
    }

    fn update(&mut self, key: MouseButton) {
        if self.next == self.to_bind.len() {
            return;
        }

        let key_to_bind = self.to_bind[self.next];
        println!("{} binded to {}", key_to_bind.to_string(), key.to_string());
        key.bind(move || {
            key_to_bind.press();
            loop {
                if key.is_pressed() == false {
                    break;
                }
            }
            key_to_bind.release();
        });

        self.next = self.next + 1;

        if self.next < self.to_bind.len() {
            println!("{}: ", self.to_bind[self.next].to_string());
        } else {
            println!("Finished configuring bindings");
        }
    }
}

fn main() {
    let bindings = Bindings::new();
    let bindings = Arc::new(Mutex::new(bindings));

    MouseButton::bind_all(move |i| {
        bindings.lock().unwrap().update(i);
    });

    inputbot::handle_input_events();
}
