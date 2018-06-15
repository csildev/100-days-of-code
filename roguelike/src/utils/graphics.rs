	#[link(name = "ncurses")]
	extern {
		fn initscr();
		fn endwin();
		fn getch() -> u8;
	}

	pub fn init(){
		unsafe { initscr(); }
	}

	pub fn end(){
		unsafe { endwin(); }
	}

	pub fn readChar() -> char {
		unsafe {
			let i = getch();
			return i as char;
		}
	}	
