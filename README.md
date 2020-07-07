# Play a video in the Task Manager

![](https://raw.githubusercontent.com/etra0/taskmgr-video/master/shrek.gif)

This is a dummy project I did to improve my understanding in Reverse Engineering.

## Usage

You need Rust, Python and `ffmpeg` with `numpy` and `pillows`.

Create a folder called `assets` then `assets/frames`, then use ffmpeg
to create the images from a video (it is recommended to use fewer frames
unless you wanna blow up your RAM)

```bash
ffmpeg -i input_video.mp4 assets/frames/%06d.png
```

Then, you need to run the Python script to generate the out.txt. The included
one is the Shrek movie.

Finally, to run the project, open a powershell in admin, `cd` into the dir
then run
```
cargo.exe build
cargo.exe run Taskmgr.exe "cargo.exe run Taskmgr.exe "<Full Path to output DLL>"
```

for example
```
cargo.exe run Taskmgr.exe "cargo.exe run Taskmgr.exe C:\Users\MyUser\taskmanager\target\debug\dll.dll"
```
