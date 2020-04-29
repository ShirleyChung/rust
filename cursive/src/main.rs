extern crate pancurses;

use pancurses::{initscr, endwin};

fn main() {
	let win = initscr();
	win.printw("Hello!");
	win.refresh();
	win.getch();
	endwin();
}
