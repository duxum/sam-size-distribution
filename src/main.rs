extern crate sized;
use sized::Sam;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;




fn read_files(file_name: &String) -> std::io::Result<()>{
	let mut file = File::open(file_name)?;
	let mut buf_reader = BufReader::new(&file);
	let mut current_sam = Sam::new(file_name);
	for line in buf_reader.lines(){
		let line = line.unwrap();
		current_sam.process_line(&line);
	}
	current_sam.print();
	println!("");
	return Ok(());

}



fn main(){
	let arguments:Vec<String> = env::args().collect();
	read_files(&arguments[1]).expect("Error opening file");
}
