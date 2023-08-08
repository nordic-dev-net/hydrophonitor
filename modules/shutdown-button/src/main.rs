//! Display message in console when a Button is pressed
// use rust_gpiozero::{Button, Debounce};
use rust_gpiozero::{Button};
// use std::time::Duration;
use std::process::Command;

fn main() {
    // Create a button which is attached to Pin 21
    let mut button = Button::new(21);
        // Add debouncing so that subsequent presses within 100ms don't trigger a press
        // .debounce(Duration::from_millis(100));

    // Add an async interrupt to trigger whenever the button is pressed
    // button
    //     .when_pressed(|_| {
    //         println!("button pressed");
    //     })
    //     .unwrap();
    println!("Waiting for shutdown button press");
    button.wait_for_press(None);
    println!("Shutdown button pressed");
    Command::new("shutdown")
        .output()
        .expect("failed to execute shutdown command");
}
