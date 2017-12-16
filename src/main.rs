
extern crate time;

#[macro_use]
extern crate clap;
use clap::App;

use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread;

const SEC_PER_MIN: u64 = 60;
const MINUETS_PER_DAY: i32 = 1440;
const DEFAULT_DIRECTORY: &str = "/usr/share/tod_wallpaper/";

/// Calculate the curent image index and the delay in seconds to the next image.
fn calculate_deltas(delay: i32) -> (usize, u64) {
    let now = time::now();
    let minuets_today = now.tm_hour * 60 + now.tm_min;
    let image_i = minuets_today / delay;
    let delay_next = delay - (minuets_today % delay);

    (image_i as usize, delay_next as u64)
}

/// Load image list from directory and calculate delay if not provided.
fn load<P: AsRef<Path>>(directory: P, delay: Option<i32>) -> (Vec<fs::DirEntry>, i32) {
    let directory = directory.as_ref();

    // Check if directory exists
    if !directory.exists() {
        eprintln!("Image directory does not exist {:?}", directory);
        std::process::exit(-1);
    }

    // Load image list
    let mut images = fs::read_dir(directory)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read image directory {:?}\n{}", directory, e);
            std::process::exit(-1);
        })
        .filter_map(|f| f.ok())
        .collect::<Vec<_>>();

    // Check that there are files
    if images.len() < 1 {
        eprintln!("No images present in {:?}", directory);
        std::process::exit(-1);
    }

    // Sort files
    images.sort_by(|a, b| a.path().cmp(&b.path()));

    // Calculate delay if not set
    // Delay is in minuets
    let delay: i32 = delay.unwrap_or(MINUETS_PER_DAY / images.len() as i32);

    (images, delay)
}

/// Application's entry point
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Get arguments
    let directory = Path::new(matches.value_of("directory").unwrap_or(DEFAULT_DIRECTORY));
    let delay = match matches.value_of("delay") {
        Some(delay) => delay.parse::<i32>().ok(),
        None => None,
    };

    let (images, delay) = load(directory, delay);

    // Main loop
    loop {
        let (image_i, next_delay) = calculate_deltas(delay);

        // Bounds checking
        let image_i = if image_i > images.len() {
            images.len() - 1
        } else {
            image_i
        };

        // Get the next image
        let next_image = &images[image_i];

        // Run feh with the next image's path
        // $ feh --bg-fill <path>
        std::process::Command::new("feh")
            .arg("--bg-fill")
            .arg(next_image.path())
            .status()
            .expect("failed to execute process: feh");

        // Sleep until the next update is due
        thread::sleep(Duration::from_secs(next_delay * SEC_PER_MIN));
    }
}
