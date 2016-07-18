use std::fs;

fn main() {
    let paths = fs::read_dir("/usr/share/tod_wallpaper/").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
