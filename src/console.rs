pub fn print_error(error: &str) {
	eprintln!("\x1b[91m{error}\x1b[0m");
}

pub fn print_warning(message: &str) {
	eprintln!("\x1b[93m{message}\x1b[0m");
}
