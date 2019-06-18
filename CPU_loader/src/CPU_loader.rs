use std::thread;

pub fn run() {
	print!("{} is prime\r", 1);
	print!("{} is prime\r", 2);
	let mut i:u128 = 3;
	loop{
  		let _handle  = thread::spawn(move || {
			let mut pass = true;
			if i != 2 && i % 2 == 0 {
				pass = false;
			}
			else {
				for j in 2..i{
					if i % j == 0 {
						pass = false;
						break;
					}			
				}

			}

			if pass{
				print!("{} is prime\r", i);
			}

		});
		//handle.join().unwrap();
		i += 2;
	}
}	