use std::process::Command;
use std::time::Duration;
use std::thread;
use std::fs;

extern crate time;

fn get_files(path: &str) -> Vec<String> {
    let mut paths = fs::read_dir(path)
                        .unwrap_or_else(|e| panic!("Could not read directory {}\n{}", path, e));
    let mut vector: Vec<String> = Vec::new();

    while let Some(Ok(p)) = paths.next() {
        let s = p.path().to_str().unwrap().to_string();
        vector.push(s);
    }

    vector.sort();
    return vector;
}

fn main() {
    let path = "/usr/share/tod_wallpaper/";
    let files = get_files(path);
    let update = 60;
    let mut last_img = -1;

    println!("Loaded {} files with delay {} from {}", files.len(), update, path);

    loop {
        let now = time::now();          // get current time
        let i = (now.tm_hour / 2) - 1;  // go to next image every two hours

        // check if the image needs to be updated
        if i != last_img {
            last_img = i;
            let v = &files[i as usize]; // get the image path

            println!("Time of day {}:{} showing {}", now.tm_hour, now.tm_min, v);

            Command::new("feh")         // run feh with the image path
                    .arg("--bg-fill")   // > feh --bg-fill [path]
                    .arg(v)
                    .status()
                    .expect("failed to execute process");
        }

        thread::sleep(Duration::new(update, 0));    // sleep for some time before next check for update
    }
}
