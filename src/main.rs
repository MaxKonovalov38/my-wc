use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn help_out() {
	// Полная справка о программе
	let mut text_hp = String::from(env!("CARGO_PKG_NAME"));
	text_hp.push_str(" (v");
	text_hp.push_str(env!("CARGO_PKG_VERSION"));
	text_hp.push_str("), ");
	text_hp.push_str(env!("CARGO_PKG_DESCRIPTION"));
	println!("{}\n##EXAMPLE:\n\tcargo run [--] [arg] [arg] ...", text_hp);
}

fn title_out() {
	// Краткое пояснение работы
	println!("My program WC!\nEXAMPLE: cargo run [--] [arg] [arg] ...");
}

fn long_out(_par: &str, _file: &str) {
	// Выполнение с доп. параметрами
	println!("Parametr: {}, File: {}", _par, _file);
}

fn simple_out(_file_name: &str) {
	// Стандартное выполнение
	if _file_name == "-h" {
		help_out();
	} else {
		println!("[ INFO ] Starting calc!");

		// переменная с типом Path
		let input_file = Path::new(_file_name); 

		// переменная для открытия файла
		let file = File::open(&input_file)
			.expect("[ ERROR ] Failed to open file!");

		// переменная содержит полное содержимое file
		let reader = BufReader::new(file);

		let mut sum = 0;

		//
		for line in reader.lines() {
			let line_cont = line.unwrap();
			sum += line_cont.len();
		}

		println!("Word count: {}", sum);
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