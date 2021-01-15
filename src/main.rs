
fn help_out() {
	// Полная справка о программе
	let mut text_hp = String::from(env!("CARGO_PKG_NAME"));
	text_hp.push_str(" (v");
	text_hp.push_str(env!("CARGO_PKG_VERSION"));
	text_hp.push_str("), ");
	text_hp.push_str(env!("CARGO_PKG_DESCRIPTION"));
	println!("{}", text_hp);
}
fn title_out() {
	// Краткое пояснение работы
	println!("My program WC!\nEXAMPLE: cargo run [--] [arg] [arg] ...");
}
fn long_out(_par: &str, _file: &str) {
	// Выполнение с доп. параметрами
	println!("Parametr: {}, File: {}", _par, _file);
}
fn simple_out(_file: &str) {
	// Стандартное выполнение
	if _file == "-h" {
		help_out();
	} else {
		println!("File: {}", _file);
	}
}

fn main() {
	// Принимаем аргументы CL
	let args: Vec<String> = std::env::args().collect();

	// Анализ аргументов
	match args.len() {
		3 => long_out(&args[1], &args[2]),
		2 => simple_out(&args[1]),
		_ => {
				println!("[ ERROR ] Invalid invocation (you done goofes!)");
				title_out();
		}
	}
}