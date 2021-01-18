use std::io::{BufRead, BufReader};

mod lib;

fn help_out() {
	// Полная справка о программе
	let mut text_hp = String::from(env!("CARGO_PKG_NAME"));
	text_hp.push_str(" (v");
	text_hp.push_str(env!("CARGO_PKG_VERSION"));
	text_hp.push_str("), ");
	text_hp.push_str(env!("CARGO_PKG_DESCRIPTION"));
	println!("{}\n", text_hp);
	title_out();
}

fn version_out() {
	let version = String::from(env!("CARGO_PKG_VERSION"));
	println!("Version program my-wc: v{}", version);
}

fn title_out() {
	// Краткое пояснение работы
	println!("EXAMPLE:\n\tcargo run [--] [param] [arg] ...");
	println!("\nPARAMETERS:\n\t'-l'  Количество строк\n\t'-c'  Количество байт\n\t'-m'  Количество символов\n\t'-L'  Количество символов в самой длинной строке\n\t'-w'  Количество слов");
}

fn long_out(_par: &str, _file: &str) {
	// Выполнение с доп. параметрами
	println!("Parametr: {}, File: {}", _par, _file);

	match _par {
		"-l" => lib::param_l(_file),
		"-c" => lib::param_c(_file),
		"-m" => lib::param_m(_file),
		"-L" => lib::param_bl(_file),
		"-w" => lib::param_w(_file),
		_ => println!("[ ERROR ] Could not verification getting parameter"),
	}
}

fn simple_out(_file_name: &str) {
	// Стандартное выполнение
	if _file_name == "-h" || _file_name == "--help" {
		help_out();
	} else if _file_name == "-v" || _file_name == "--version" {
		version_out();
	} else {
		println!("[ INFO ] Starting calc!");

		let mod_file = lib::open_file(_file_name);

		// переменная содержит полное содержимое file
		let reader = BufReader::new(mod_file);

		let mut sum_chars = 0;
		let mut sum_lines = 0;

		//
		for line in reader.lines() {
			let line_cont = line.unwrap();

			sum_chars += line_cont.len();
			sum_lines += 1;
		}

		println!(" {}\tlen strings\t{}\t{}", sum_lines, sum_chars, _file_name);
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