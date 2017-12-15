# Time of Day Wallpaper `tod_wallpaper`

This program will update your wallpaper based on the time of day. Currently, every **2** hours, the wallpaper will switch to the next of **12** wallpapers in the directory `/usr/shared/tod_wallpapers/`

## Usage

### Install

```
$ git clone https://github.com/twh2898/tod_wallpaper.git
$ cd tod_wallpaper/
$ ./build
$ sudo ./install
```

### Use

```
$ tod_wallpaper
```

## Dependencies

`feh` is used to set the wallpaper on an Xorg desktop

## Config

Loaded from
* `~/.config/tod_wallpaper/config`
* `~/.tod_wallpaper/config`
* `/etc/tod_wallpaper/config`

* picture directory
* hour seperation

## Todo

- [ ] Load image folder path from config file
- [ ] Check for correct number of images
- [ ] Load image to time mapping from config file
- [ ] Create an install script
- [ ] Add command line arguments (--set_img_path, --set_update_interval)
