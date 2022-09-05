use std::env;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct State {
	pc: usize,
	accum: usize,
	mbox: [usize; 100],
	neg_flag: bool,
	reg: [usize; 6],
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PC: {}, accum: {}, neg_flag: {}, reg: {:?}, mbox: ", self.pc, self.accum, self.neg_flag, self.reg)?;
	for i in 0..10 {
		let l = i*10;
		for j in l..l+10 {
			write!(f, "{}:{}\t", j, self.mbox[j]);
		}
		writeln!(f, "");
	}
	Ok(())
    }
}

fn load(file_path: &String) -> State {
	let file = fs::read_to_string(file_path);
	let prog = file.unwrap();
	let vec: Vec<&str> = prog.split("\n").collect();
	let n = vec.len();

	let mut state = State {
		pc:0,
		accum:0,
		mbox:[0;100],
		neg_flag:false,
		reg:[0;6],
	};

	for i in 0..n {
		if vec[i]=="" {
			break;
		}
		state.mbox[i] = vec[i].parse::<usize>().unwrap();
	}
	
	return state;
}

/* Returns if the program has finished */
fn run(state: &mut State) -> bool {

	loop {

		let a = state.mbox[state.pc]/1000;
		let b = state.mbox[state.pc]/100 - 10*a;
		let xx = state.mbox[state.pc]%100;

		if a==1 {
			if b==9 {
				state.accum = state.accum + state.mbox[xx];
				state.neg_flag = false;
			}else {
				let mut sum = 0;
				for i in 0..(b+1){
					sum = sum + state.reg[i];
				}
				state.accum = state.accum + sum;
				state.neg_flag = false;
			}
		}

		else if a==2 {
			if b==9 {
				if state.mbox[xx] >= state.accum {
					state.accum = state.mbox[xx] - state.accum;
				}else {
					state.accum = 0;
					state.neg_flag = true;
				}
				
			}else {
				let mut sum = 0;
				for i in 0..(b+1){
					sum = sum + state.reg[i];
				}
				if state.accum >= sum {
					state.accum = state.accum - sum;
				}else {
					state.accum = 0;
					state.neg_flag = true;
				}				
			}
		} 

		else if a==3 {
			state.mbox[xx] = state.accum;
		} 

		else if a==5 {
			if b==9 {
				state.accum = state.mbox[xx];
			}else {
				for i in 0..(b+1){
					state.reg[i] = state.mbox[xx+i];
				}
			}
		} 

		else if a==6 {
			state.pc = xx;
			continue;
		} 
		else if a==7 {
			if b==0 {
				if state.accum == 0 {
					state.pc = xx;
					continue;
				}
			}else {
				if state.neg_flag == false{
					state.pc = xx;
					continue;
				}
			}
		} 

		else if a==9{
			if xx==1{
				let mut input = String::new();
				std::io::stdin().read_line(&mut input);
				let vec: Vec<&str> = input.split("\n").collect();
				state.accum = vec[0].parse::<usize>().unwrap();

			} else if xx==2 {
				println!("{}", state.accum);
			} else {
				println!("{}", state);
			}
		} 
		else {
			break;
		}
		state.pc += 1;
	}
	true
}
 
fn main() -> Result<(), String> {
	let args: Vec<String> = env::args().collect();
	let file_path = args.get(1).ok_or("Required file path")?;

	// Load the program
	let mut state = load(file_path);
		
	// Run the program
	run(&mut state);
	
	Ok(())
}