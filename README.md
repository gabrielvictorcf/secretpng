# secretpng
<img src="https://i.imgur.com/u0lJRTj.png" alt="img" align="right" width="400px">

Secretpng is a small PNG encoder/decoder CLI that can "hide" messages inside the file without changing in
any way how the image is read or displayed by common editors.

This is a toy project made just to play with Rust and, of course, learn more about it and it's development cycle,
so, why not make a fun CLI!

And yes, this crab image does have a message inside it :P

# Installing
Install through cargo with
```
cargo install --git https://github.com/gabrielvictorcf/secretpng secretpng
```

# Usage
Here are the commands available:
```
secretpng encode <image_path> <chunk_type> <message> -out_file # secretpng encode dice.png ruSt secretmsg

secretpng decode <img_path> <chunk_type> # secretpng decode dice.png ruSt

secretpng remove <img_path> <chunk_type> # secretpng remove dice.png ruSt

secretpng print <img_path> # secretpng print dice.png
```