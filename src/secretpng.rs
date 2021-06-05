use std::path::PathBuf;
use std::str::FromStr;
use std::fs;

use clap::{App,Arg};
use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;

pub fn encode(fp: PathBuf,chunk_type: &str,msg: &str,out: Option<PathBuf>) {
	let mut png = match Png::from_file(&fp) {
		Ok(png_struct) => png_struct,
		Err(e) => {
			eprintln!("Encode error: could not read png input file - {}",e);
			std::process::exit(1);
		},
	};
	
    let chunk_type = match ChunkType::from_str(chunk_type) {
        Ok(c_type) => c_type,
        Err(e) => {
			eprintln!("Encode error: invalid chunk_type - {}",e);
			std::process::exit(1);
		},
    };
    let data = Vec::from(msg);
    let new_chunk = Chunk::new(chunk_type,data);

    png.append_chunk(new_chunk);	// Currently all messages are appended to file end

	let fp = out.unwrap_or(fp);
	if let Err(e) = fs::write(fp, png.as_bytes()) {
		eprintln!("Encode error: aborting on file write - {}",e);
		std::process::exit(1);
	}
}

pub fn decode(fp: PathBuf,chunk_type: &str) {
	let png = match Png::from_file(&fp) {
		Ok(png_struct) => png_struct,
		Err(e) => {
			eprintln!("Decode error: could not read png input file - {}",e);
			std::process::exit(1);
		},
	};
	
	match png.chunk_by_type(chunk_type) {
		Some(chunk) => println!("Chunk found!\nChunk : {}",chunk),
		None => println!("Could not find target chunk"),
	};
}

pub fn remove(fp: PathBuf,chunk_type: &str) {
	let mut png = match Png::from_file(&fp) {
		Ok(png_struct) => png_struct,
		Err(e) => {
			eprintln!("Remove error: could not read png input file - {}",e);
			std::process::exit(1);
		},
	};
	match png.remove_chunk(chunk_type) {
    	Ok(chunk) => println!("Chunk removed sucessfully!\nChunk: {}",chunk),
    	Err(e) => {
			println!("Remove Unsuccessful: {}",e);
			return;
		}
	}

	if let Err(e) = fs::write(fp, png.as_bytes()) {
		eprintln!("Remove error: aborting on saving changes - {}",e);
		std::process::exit(1);
	}
}

pub fn print(fp: PathBuf) {
	match Png::from_file(&fp) {
		Ok(png) => println!("Image {:?}\n{}",fp,png),
		Err(e) => {
			eprintln!("Print error: could not read png input file - {}",e);
			std::process::exit(1);
		},
	};
}

pub fn secretpng() {
    let secretpng_app = App::new("secretpng")
        .version("0.1.0")
        .author("Gabriel Victor <gabrielvcf@outlook.com>")
        .about("secretpng is a png encoder/decoder that can hide, find or remove a message inside a png")
		.setting(clap::AppSettings::ColoredHelp)
		.arg(Arg::new("operation")
			.index(1)
			.required(true)
			.requires("image")
			.possible_values(&["encode","decode","remove","print"]))//.get_arguments().for_each(|f| println!("{}",f));
		.arg(Arg::new("image")
			.about("Image filepath")
			.index(2)
			.takes_value(true))
        .arg(Arg::new("type")
			.about("Chunk type")
			.index(3)
			.required_if_eq_any(&[("operation","encode"),("operation", "decode"),("operation", "remove")])
			.takes_value(true))
        .arg(Arg::new("message")
			.about("Chunk data to be encoded")
			.index(4)
			.required_if_eq("operation", "encode")
			.takes_value(true))
        .arg(Arg::new("out")
			.about("output file")
			.index(5)
			.takes_value(true))
		.get_matches();

	// This block guarantees that image argument is present and contains a valid Path
	let img_path: PathBuf = secretpng_app.value_of_t_or_exit("image");
	if !img_path.is_file() {
		eprintln!("Secretpng error: input image does not exist (invalid file path)");
        std::process::exit(1);
	}

	let chunk_type = secretpng_app.value_of_lossy("type").unwrap_or_default().into_owned();

	match secretpng_app.value_of("operation").unwrap() {
		"encode" => {
			let msg = secretpng_app.value_of_lossy("message").unwrap().into_owned();
			let out: Option<PathBuf> = secretpng_app.value_of_t("out").ok();
			encode(img_path, &chunk_type, &msg, out);
		}
		"decode" => decode(img_path, &chunk_type),
		"remove" => remove(img_path, &chunk_type),
		"print" => print(img_path),
		_ => ()
	}
}