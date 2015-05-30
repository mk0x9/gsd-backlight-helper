#![feature(convert)]

use std::env;
use std::process;

mod helper;

const OPT_SET:     &'static str = "--set-brightness";
const OPT_GET:     &'static str = "--get-brightness";
const OPT_GET_MAX: &'static str = "--get-max-brightness";

static BACKLIGHT_PATH: &'static str = "/sys/class/backlight/intel_backlight";

fn show_help() {
    println!("Possible options:");
    println!("  {}", OPT_SET);
    println!("  {}", OPT_GET);
    println!("  {}", OPT_GET_MAX);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        show_help();
        process::exit(0);
    }
    let backlight = helper::Backlight::new(BACKLIGHT_PATH);
    match args[1].as_str() {
        OPT_SET => {
            if args.len() < 3 {
                println!("wrong option");
                process::exit(1);
            }
            let value = args[2].parse::<u32>().unwrap();
            backlight.set(value);
        },
        OPT_GET => println!("{}", backlight.get()),
        OPT_GET_MAX => println!("{}", backlight.get_max()),
        _ => {
            println!("wrong option");
            process::exit(1);
        }
    }
}
