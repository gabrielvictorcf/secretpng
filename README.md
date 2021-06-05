# secretpng
<img src="./crabby/lilcrab.png" alt="img" align="right" width="400px">

Secretpng is a small, PNG encoder/decoder, cli tool  that can "hide" messages inside a file without changing, in
any way, how the image is read or displayed by common viewers.

This is a toy project made to play with Rust and, of course, learn more about it and it's development cycle,
so, why not make a fun CLI!

And yes, this crab image does have a message inside it :P

# Installing
Install through cargo with
```
cargo install --branch main --git https://github.com/gabrielvictorcf/secretpng secretpng
```

# Usage
Every command takes a path to a png file and, probably, a `chunk_type` argument which is a utf-8 string
of length 4. Here's what's available:
```
# Encode - encode any utf-8 message and optionally save in an output file
secretpng encode <image_path> <chunk_type> <message> -out_file # secretpng encode crab.png ruSt secretmsg

# Decode - find the message hidden in the chunk of chunk_type
secretpng decode <img_path> <chunk_type> # secretpng decode crab.png ruSt

# Remove - remove the chunk with chunk_type from the image
secretpng remove <img_path> <chunk_type> # secretpng remove crab.png ruSt

# Print - print all chunks in an image, data is truncated to 60 for better visibility
secretpng print <img_path> # secretpng print crab.png
```