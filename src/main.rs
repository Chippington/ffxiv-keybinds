use inputbot::{
    KeySequence,
    KeybdKey::{self, *},
    MouseButton::{self, *},
};

fn main() {
    MouseButton::bind_all(|i| {
        println!("Pressed: {}", i.to_string());
        loop {
            if i.is_pressed() == false {
                break;
            }
        }
        println!("Released: {}", i.to_string());
    });

    // println!("Hello, world!");
    // MouseButton::X1Button.bind(|| {
    //     println!("A");
    //     loop {
    //         if MouseButton::X1Button.is_pressed() == false {
    //             break;
    //         }
    //     }
    //     println!("B");
    // });

    // MouseButton::X2Button.bind(|| {
    //     println!("C");
    //     loop {
    //         if MouseButton::X2Button.is_pressed() == false {
    //             break;
    //         }
    //     }
    //     println!("D");
    // });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
