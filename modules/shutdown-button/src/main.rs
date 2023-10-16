use clap::Parser;
use log::info;
use rust_gpiozero::Button;
use std::process::Command;
use std::time::Instant;

#[derive(Parser)]
#[clap(version = "1.0", author = "Satu Koskinen")]
struct Cli {
    /// GPIO pin that the button is connected to
    #[clap()]
    gpio: u8,

    /// Button press duration in seconds to trigger shutdown
    #[clap()]
    duration: u32,
}

fn main() {
    env_logger::init();
    let args = Cli::parse();

    info!("Starting shutdown button on GPIO {}", args.gpio);
    let mut button = Button::new(args.gpio);

    loop {
        info!(
            "Waiting for shutdown button press, press for longer than {:?} to shutdown",
            args.duration
        );
        button.wait_for_press(None);
        let press_start = Instant::now();
        info!("Shutdown button pressed");
        button.wait_for_release(None);
        let duration = press_start.elapsed();
        info!("Shutdown button released after {:?}", duration);
        if duration.as_secs_f32() > args.duration as f32 {
            info!(
                "Shutdown button pressed for longer than {:?}, shutting down",
                args.duration
            );
            Command::new("shutdown")
                .output()
                .expect("failed to execute shutdown command");
            break;
        }
    }
}
