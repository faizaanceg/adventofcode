use std::io::prelude::*;
use std::fs::File;

fn main() {

	let mut string = String::new();
	
	//Under the assumption that no errors will take place,
	//I have not used try!
	File::open("input.txt").unwrap().read_to_string(&mut string);

	let floor = string.chars().fold((0, 0, -1), |mut f, c| {

		// The actual floor value
		f.0 = f.0 + if c == '(' {1} else {-1};

		// The no.of brackets encountered
		f.1 = f.1 + 1;
		
		// Deciding if the first basement lvl is reached
		if f.0 == -1 && f.2 == -1 { f.2 = f.1 };
			
		f
	});

	// Part 1
	println!("Santa's final destination: {:?}", floor.0);

	//Part 2
	println!("The position of character when Santa enters basement for first time: {}", floor.2);
}












































