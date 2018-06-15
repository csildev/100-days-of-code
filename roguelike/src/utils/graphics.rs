	#[link(name = "ncurses")]
	extern {
		fn initscr();
		fn endwin();
		fn getch() -> u8;
		fn mvaddch(y:i64, x:i64, c:u8);
		
		fn refresh();
		fn clear();

		fn curs_set(on: i64);
		fn cbreak();
		fn nocbreak();
		fn echo();
		fn noecho();
	}

	pub fn addch(x:i64, y:i64, c: char){
		unsafe { 
			mvaddch(y,x, c as u8);
			refresh();	
		}
	}

	pub fn win_clear(){
		unsafe { clear(); }
	}

	pub fn init(){
		unsafe { 
			initscr();
			noecho();
			curs_set(0);
			cbreak();
		 }
	}

	pub fn end(){
		unsafe { 
			echo();
			curs_set(1);
			nocbreak();
			endwin(); 
		}
	}

	pub fn read_char() -> char {
		unsafe {
			let i = getch();
			return i as char;
		}
	}	
