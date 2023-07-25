mod brightness;
mod volume;
mod playback;
mod keymap;

use clap::{Parser, Subcommand};

/// Media control program for ME!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {

    /// Mode
    #[clap(subcommand)]
    operation: Operation,
}

#[derive(Subcommand, Debug)]
enum Operation {
    /// Control brightness
    Brightness {
        /// Send notification
        #[clap(short, long, action)]
        alert: bool,

        /// Get current brightness
        #[clap(short, long, action)]
        get: bool,

        /// Get max brightness
        #[clap(short = 'm', long, action)]
        get_max: bool,

        /// Set brightness
        #[clap(short, long, value_parser, value_name = "U32")]
        set: Option<i32>,

        /// Increment brightness
        #[clap(short, long, value_parser, value_name = "U32")]
        increment: Option<i32>,

        /// Decrement brightness
        #[clap(short, long, value_parser, value_name = "U32")]
        decrement: Option<i32>,
    },

    /// Control volume
    Volume {
        /// Send notification
        #[clap(short, long, action)]
        alert: bool,

        /// Get volume
        #[clap(short, long, action)]
        get: bool,

        /// Set volume
        #[clap(short, long, value_parser, value_name = "U32")]
        set: Option<i32>,

        /// Increment volume
        #[clap(short, long, value_parser, value_name = "U32")]
        increment: Option<i32>,

        /// Decrement volume
        #[clap(short, long, value_parser, value_name = "U32")]
        decrement: Option<i32>,
    },

    /// Control keymap
    Keymap {
        /// Send notification
        #[clap(short, long, value_parser, value_name = "NAME")]
        alert: String,
    },

    /// Control playback
    Playback {
        /// Send notification
        #[clap(short, long, action)]
        alert: bool,

        /// Skip to next track
        #[clap(short, long, action)]
        next: bool,

        /// Skip to previous track
        #[clap(short, long, action)]
        previous: bool,

        /// Toggle between play and pause
        #[clap(short, long, action)]
        toggle: bool,
    }
}


fn main() {
    let args = Args::parse();

    match &args.operation {
        Operation::Brightness { get, get_max, set, increment, decrement, alert } => {
            let cur_brightness = brightness::get_current();
            let mut new_brightness = cur_brightness;

            if let Some(set) = set {
                new_brightness = *set;
            }

            if let Some(increment) = increment {
                new_brightness += *increment;
            }

            if let Some(decrement) = decrement {
                new_brightness -= *decrement;
            }

            if new_brightness != cur_brightness {
                brightness::set(new_brightness);
            }

            if *get {
                println!("{}", new_brightness);
            }

            if *get_max {
                println!("{}", brightness::get_max());
            }

            if *alert {
                brightness::send_notification(new_brightness);
            }
        }

        Operation::Volume { get, set, increment, decrement, alert }=> {
            println!("mediacuddle value was used!");
            println!("Get: {:?}", get);
            println!("Set: {:?}", set);
            println!("Inc: {:?}", increment);
            println!("Dec: {:?}", decrement);
            println!("Alr: {:?}", alert);
            if *get {
                volume::get_volume();
            }
        }

        Operation::Playback { next, previous, toggle, alert }=> {
            if *toggle {
                playback::toggle();
            }

            if *next {
                playback::next();
                // Wait for media player to play next song in case of skip
                std::thread::sleep(core::time::Duration::from_millis(50));
            }

            if *previous {
                playback::prev();
                // Wait for media player to play next song in case of skip
                std::thread::sleep(core::time::Duration::from_millis(50));
            }

            if *alert {
                playback::send_notification();
            }
        }

        Operation::Keymap { alert } => {
            keymap::send_notification(alert);
        },
    };
}

