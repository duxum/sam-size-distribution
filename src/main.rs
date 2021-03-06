extern crate sized;
use sized::Sam;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufWriter, BufReader};
use std::io::BufRead;




/*fn read_sam(file_name: &String) -> std::io::Result<()>{
	let mut file = File::open(file_name)?;
	let mut buf_reader = BufReader::new(&file);
	let mut current_sam = Sam::new(file_name);
	for line in buf_reader.lines(){
		let line = line.unwrap();
		current_sam.process_line(&line);
	}
	println!("{}", current_sam);
	println!("");
	return Ok(());

}*/

fn read_files(file_name:&String, to: &BufWriter<&File>){
	let mut file = File::open(file_name).unwrap();
	let mut buf_reader = BufReader::new(&file);
	for file in buf_reader.lines(){
		let file = file.unwrap();
		let mut current_sam = Sam::new(&file);
		current_sam.process();
	}			
} 

fn main(){
	let arguments:Vec<String> = env::args().collect();
	let mut file = File::create(&arguments[2]).unwrap();
	let mut stream = BufWriter::new(&file);
	read_files(&arguments[1], &stream);
	//let mut stream = BufWriter::new(&file);
	//stream.write(String::from("Dukunde")).unwrap();
	//read_files(&arguments[1]).expect("Error opening file");
}
