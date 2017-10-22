use std::ops::Index;
use std::collections::HashMap;





#[derive(Debug)]
pub struct polality<'a>{
	data: HashMap<usize, HashMap<&'a str, HashMap<u8, Stored>>>
}



#[derive(Clone, Debug)]
struct Stored(usize, usize);


impl<'a> polality<'a>{
	fn new(size: usize) -> polality<'a>{
		let mut initials = Stored(0, 0);
		let mut nucleotide_handling = HashMap::new();
		nucleotide_handling.insert('A' as u8, initials.clone()); nucleotide_handling.insert('G' as u8, initials.clone());
		nucleotide_handling.insert('C' as u8, initials.clone()); nucleotide_handling.insert('T' as u8, initials.clone());

		let mut polality = HashMap::new();
		polality.insert("single", nucleotide_handling.clone()); polality.insert("multiple", nucleotide_handling.clone());		
		
		let mut mismatch = HashMap::new();
		mismatch.insert(0, polality.clone()); mismatch.insert(1, polality.clone());
		mismatch.insert(2, polality.clone()); mismatch.insert(3, polality.clone());
		polality{ data: mismatch}
		
	}

}


#[derive(Debug)]
pub struct Sam<'a>{
	name: &'a String,
	nucleotides: Vec<usize>,
	pub positive: HashMap<usize, polality<'a>>,
	pub negative: HashMap<usize, polality<'a>>,
	

}

impl<'a> Sam<'a>{
	pub fn new(name: &String ) -> Sam{
		Sam{name: name, nucleotides: Vec::new(), positive: HashMap::new(), negative: HashMap::new()}
	}
	pub fn has(&self, size: &usize) -> bool{
		self.nucleotides.contains(size)
	}

	pub fn add(&mut self, size:&usize){
		self.positive.insert(*size, polality::new(*size));
		self.negative.insert(*size, polality::new(*size));
		self.nucleotides.push(*size);
		
	}

	pub fn print(&self){
		println!("{:?}" ,self.positive);
		println!("{:?}", self.positive);


	}

	pub fn process_line(&mut self, line:&String){
		let results:Vec<&str> = line.split("\t").collect();
		if results.len()<=12{
		//	println!("Passed here");
			return ();
		}
	//	println!("Not threr");
                let sequence = results[9].as_bytes();
                let current_nucleotide_size = sequence.len();
                if !self.has(&current_nucleotide_size){
//                        println!("\n{} is not present ", current_nucleotide_size);
                        self.add(&current_nucleotide_size);
                }
                let polality = results[1].parse::<usize>().unwrap();
                let first_column:Vec<&str> = results[0].split(":").collect();
		let mismatch = results[13].split(":").collect::<Vec<&str>>()[2].parse::<usize>().unwrap();
                let abundance:usize = first_column[1].parse().unwrap();

                if polality == 0 || polality ==256{
			let nucleotide_of_interest = sequence[0];
			if polality ==0{
				//println!("{:?}", self);
				//println!("Nucleotide:{}, Mismatch: {}, Available Nucleotides: {:?}", nucleotide_of_interest as char, polality, self.nucleotides);
                       		//print!("{:?}", self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&polality).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().0);
                       		self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().0+=1;
                       		self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().1+=abundance;
				//println!("");
				//return
			}else{
                       		self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("multiple").unwrap().get_mut(&nucleotide_of_interest).unwrap().0+=1;
                       		self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("multiple").unwrap().get_mut(&nucleotide_of_interest).unwrap().1+=abundance;
			}
		}else{
			let nucleotide_of_interest = sequence[current_nucleotide_size-1];
			if polality ==16{
//				println!("First Column is {:?} Nucleotide:{}, Mismatch: {}, Available Nucleotides: {:?}", first_column, nucleotide_of_interest as char, mismatch, self.nucleotides);
  //                     		print!("{:?}\n", self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap());
//				println!("Results is {:?}", results);
                       		//print!("{:?}", self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().0);
                       		self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().0+=1;
                       		self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().1+=abundance;
			}else{
                      	 	self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("multiple").unwrap().get_mut(&nucleotide_of_interest).unwrap().0+1;
                       		self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("multiple").unwrap().get_mut(&nucleotide_of_interest).unwrap().1+=abundance;
				}
		}

	}
	
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
