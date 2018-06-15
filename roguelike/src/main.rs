mod utils;
use utils::graphics;


fn main() {
	graphics::init();
	graphics::win_clear();
	graphics::addch(5,5,'@');
	graphics::read_char();
	graphics::end();
}
