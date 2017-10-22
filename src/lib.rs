use std::collections::HashMap;
use std::fmt;

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
	fn query(&mut self, mismatch:&usize, s_or_m:&str, nucleotide:&u8) -> &mut Stored {
		self.data.get_mut(&mismatch).unwrap().get_mut(s_or_m).unwrap().get_mut(nucleotide).unwrap()
	} 
	fn query_mut(&mut self, mismatch:&usize, s_or_m:&str, nucleotide:&u8) -> &mut Stored {
		self.data.get_mut(&mismatch).unwrap().get_mut(s_or_m).unwrap().get_mut(nucleotide).unwrap()
	} 
}

impl<'a> fmt::Display for polality<'a>{
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
		write!(f, "Called");
		Ok(())
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
			return ();
		}
                let sequence = results[9].as_bytes();
                let current_nucleotide_size = sequence.len();
                if !self.has(&current_nucleotide_size){
                        self.add(&current_nucleotide_size);
                }
                let polality = results[1].parse::<usize>().unwrap();
                let first_column:Vec<&str> = results[0].split(":").collect();
		let mismatch = results[13].split(":").collect::<Vec<&str>>()[2].parse::<usize>().unwrap();
                let abundance:usize = first_column[1].parse().unwrap();

                if polality == 0 || polality ==256{
			let nucleotide_of_interest = sequence[0];
			if polality ==0{
                       		//self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().0+=1;
				self.positive.get_mut(&current_nucleotide_size).unwrap().query_mut(&mismatch, "single", &nucleotide_of_interest).0+=1;
				self.positive.get_mut(&current_nucleotide_size).unwrap().query_mut(&mismatch, "single", &nucleotide_of_interest).1+=abundance;
                       		//self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().1+=abundance;
			}else{
                       		self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("multiple").unwrap().get_mut(&nucleotide_of_interest).unwrap().0+=1;
                       		self.positive.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("multiple").unwrap().get_mut(&nucleotide_of_interest).unwrap().1+=abundance;
			}
		}else{
			let nucleotide_of_interest = sequence[current_nucleotide_size-1];
			if polality ==16{
                       		self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().0+=1;
                       		self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("single").unwrap().get_mut(&nucleotide_of_interest).unwrap().1+=abundance;
			}else{
                      	 	self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("multiple").unwrap().get_mut(&nucleotide_of_interest).unwrap().0+1;
                       		self.negative.get_mut(&current_nucleotide_size).unwrap().data.get_mut(&mismatch).unwrap().get_mut("multiple").unwrap().get_mut(&nucleotide_of_interest).unwrap().1+=abundance;
				}
		}

	}




	pub fn write_for_excel(&self){
		println!("{}", self.name);
		println!("Positive");
		println!("0,,,,,,,,1,,,,,,,,2,,,,,,,,3,,,,,,,,");
		println!("A,,G,,C,,T,,A,,G,,C,,T,,A,,G,,C,,T,,A,,G,,C,,T,,");
		println!("Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,Unique,Abund,");

	}
	
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
