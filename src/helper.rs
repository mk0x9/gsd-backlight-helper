use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

const BRIGHTNESS:     &'static str = "/brightness";
const BRIGHTNESS_MAX: &'static str = "/max_brightness";

pub struct Backlight<'a> {
    path: &'a str,
}

impl<'a> Backlight<'a> {
    pub fn new(path: &str) -> Backlight {
        Backlight {
            path: path,
        }
    }

    fn get_with_suffix_path(&self, suffix_path: &str) -> u32 {
        let fp = format!("{}{}", self.path, suffix_path);
        let mut f = File::open(Path::new(&fp)).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        return s.split('\n')
            .next().unwrap()
            .parse::<u32>().unwrap();
    }

    fn set_with_suffix_path(&self, suffix_path: &str, value: u32) {
        let fp = format!("{}{}", self.path, suffix_path);
        let mut opts = OpenOptions::new();
        opts.write(true);
        let mut f = opts.open(Path::new(&fp)).unwrap();
        f.write_fmt(format_args!("{}", value)).unwrap();
    }

    pub fn get(&self) -> u32 {
        return self.get_with_suffix_path(BRIGHTNESS);
    }

    pub fn set(&self, value: u32) {
        self.set_with_suffix_path(BRIGHTNESS, value);
    }

    pub fn get_max(&self) -> u32 {
        return self.get_with_suffix_path(BRIGHTNESS_MAX);
    }
}
