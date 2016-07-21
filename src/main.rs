use std::process::Command;
use std::time::Duration;
// use std::io::prelude::*;
// use std::fs::File;
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

struct Config{
    path: String,
    files: Vec<String>,
    update: u64
}

/*  Broken  */
// fn get_config() -> Result<Config, String> {
//     let mut f = File::open("config").unwrap_or_else(|e| Err(String::from("Error loading file!")));
//     let mut s = String::new();
//     f.read_to_string(&mut s).expect("Could not read file");
//     println!("Read {} lines", s.len());
//     let path: String = String::from("/usr/share/tod_wallpaper/");
//     // let files = get_files(&path);
//     let files: Vec<String> = vec![];
//     let update = 60;
//
//     // let mut iter = s.split(|c| c == '\n').filter(|line| line.chars().next() != b'#');
//     for line in s.split('\n') {
//         if line.chars().next().unwrap() != '#' || line.split_whitespace().count() < 1{
//             continue;
//         }
//
//         for obj in line.split_whitespace() {
//             println!("{}", obj);
//         }
//         println!("");
//     }
//
//     Ok(Config{
//         path: path,
//         files: files,
//         update: update
//     });
// }

fn main() {
    // let conf = get_config().unwrap();
    let conf = Config {
        path: String::from("/usr/share/tod_wallpaper/"),
        files: get_files("/usr/share/tod_wallpaper/"),
        update: 60
    };
    let mut last_img = -1;

    println!("Loaded {} files with delay {} from {}", conf.files.len(), conf.update, conf.path);

    loop {
        let now = time::now();          // get current time
        let i = now.tm_hour / 2;        // go to next image every two hours

        // check if the image needs to be updated
        if i != last_img {
            last_img = i;
            let v = &conf.files[i as usize]; // get the image path

            println!("Time of day {}:{} showing {}", now.tm_hour, now.tm_min, v);

            Command::new("feh")         // run feh with the image path
                    .arg("--bg-fill")   // > feh --bg-fill [path]
                    .arg(v)
                    .status()
                    .expect("failed to execute process");
        }

        thread::sleep(Duration::new(conf.update, 0));    // sleep for some time before next check for update
    }
}
