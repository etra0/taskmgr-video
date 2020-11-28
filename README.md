# Play a video in the Task Manager

![](https://raw.githubusercontent.com/etra0/taskmgr-video/master/shrek.gif)

This is a dummy project I did to improve my understanding in Reverse Engineering.

## Usage

You need Rust, Python with `numpy` and `pillows` and `ffmpeg` .
**Only tested with Windows 10 Version 1909**

Create a folder called `assets` then `assets/frames`, then use ffmpeg
to create the images from a video (it is recommended to use fewer frames
unless you wanna blow up your RAM)

```bash
ffmpeg -i input_video.mp4 assets/frames/%06d.png
```

Then, you need to run the Python script to generate the out.txt. The included
one is the Shrek movie. **out.txt must be in the root folder of the project**

Finally, open a powershell in admin, `cd` into the dir
and run
```bash
cargo.exe r
```
