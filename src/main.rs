use std::fs;

mod lib;
mod params;

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
	println!("EXAMPLE:\n\tcargo run -- [param] [file] ...");
	println!("\nPARAMETERS:\n\t'-v, --version'  Версия программы\n\t'-h, --help'  Документация программы");
	println!("\t'-l'  Количество строк\n\t'-c'  Количество байт\n\t'-m'  Количество символов\n\t'-L'  Количество символов в самой длинной строке\n\t'-w'  Количество слов");
}

fn long_out(_par: &str, _file: &str) {
	// Выполнение с доп. параметрами

	// Пока не пойму почему mut!!!
	let mut mod_file = lib::open_file(_file);

	match _par {
//		"-l" => params::param_l(mod_file),
		"-c" => println!("{}   {}", params::param_c(&mut mod_file), _file),
//		"-m" => params::param_m(mod_file),
//		"-L" => params::param_bl(mod_file),
//		"-w" => params::param_w(mod_file),
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
		// Переменные для вывода инфы
		let mut sum_l = 0;
		let mut sum_w = 0;
		let mut sum_m = 0;

		let contents = fs::read_to_string(_file_name)
			.expect("Something went wrong reading the file");

		for line in contents.lines() {
			sum_l += 1;
			sum_m += line.len();
		}
		if sum_l < 6 {
			sum_l -= 1;
		}
		sum_m += sum_l;
		for _ in contents.split_whitespace() {
			sum_w += 1;
		}

		println!(" {} {} {} {}", sum_l, sum_w, sum_m, _file_name);
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