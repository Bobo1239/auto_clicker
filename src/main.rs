extern crate inputbot;

use inputbot::*;

static mut RUN: bool = false;

fn main() {
    KeybdKey::RControlKey.bind(|| unsafe {
        if RUN {
            RUN = false;
        } else {
            RUN = true;

            while RUN {
                MouseButton::LeftButton.press();
                MouseButton::LeftButton.release();
                // std::thread::sleep_ms(50);
            }
        }
    });

    handle_input_events();
}
