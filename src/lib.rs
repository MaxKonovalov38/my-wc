use std::path::Path;
use std::fs::File;

pub fn open_file(file: &str) -> File {
	// Работа с файлом
	// переменная с типом Path
	let input_file = Path::new(file);
	// переменная для открытия файла
	let file = File::open(&input_file)
		.expect("[ ERROR ] Failed not open file!");

	file
}

// TO-DO
pub fn param_l(file: &str) {
	// Количество строк
}

pub fn param_c(file: &str) {
	// Количество байт
}

pub fn param_m(file: &str) {
	// Количество символов
}

pub fn param_bl(file: &str) {
	// Количество символов в самой длинной строке
}

pub fn param_w(file: &str) {
	// Количество слов
}