# Time of Day Wallpaper `tod_wallpaper`

This program will update your wallpaper based on the time of day. Currently, every **2** hours, the wallpaper will switch to the next of **12** wallpapers in the directory `/usr/shared/tod_wallpapers/`

## Install

```
$ git clone https://github.com/twh2898/tod_wallpaper.git
$ cd tod_wallpaper/
$ ./build
$ sudo ./install
```

## Useage

### Command Line
```
$ tod_wallpaper
```

### Systemd
```
$ sudo systemctl enable tod_wallpaper
$ sudo systemctl start tod_wallpaper
```

## Dependencies

`feh` is used to set the wallpaper on an Xorg desktop

## Todo

- [x] Load image folder path from command line
- [x] Create an install script
- [x] Add command line arguments (-t &lt;delay&gt;, -d &lt;directory&gt;)
- [ ] Use library instead of feh

## Credits

Images taken from http://www.bitday.me/
