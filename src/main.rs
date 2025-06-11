use std::env;
use std::fs::{File, self};
use std::process;

mod calling_help;
mod calling_version;
mod lib_param;

/// Выполнение программы (число символов новой строки, слов и байт)
fn working_a_file(file_name: &str) {
    // Переменные для вывода информации
    let mut sum_l = 0;    // количество строк
    let mut sum_w = 0;    // количество слов
    let mut sum_m = 0;    // количество символов

    let contents = fs::read_to_string(file_name)
        .expect("[ERROR] -- Что-то пошло не так при чтении файла");

    for line in contents.lines() {
        sum_l += 1;
        sum_m += line.len();
    }
    sum_l -= 1;
    sum_m += sum_l;

    for _ in contents.split_whitespace() {
        sum_w += 1;
    }

    println!("{} {} {} {}", sum_l, sum_w, sum_m, file_name);
}

/// Проверка что file_name является файлом
fn file_verification(file_name: &str) -> File {
    let file_result = File::open(file_name);

    let file = match file_result {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[ERROR] -- Ошибка открытия файла: {}", e);
            process::exit(1);
        }
    };

    file
}

/// Выполнение программы с одним аргументом
fn single_argument_processing(param: &str) {
    match param {
        // Вывод полной справки
        "-h" | "--help" => calling_help::help_out(),
        // Вывод версии программы
        "-v" | "--version" => calling_version::version_out(),
        // Выполнение программы (число символов новой строки, слов и байт)
        _ => {
            file_verification(param);
            working_a_file(param);
        }
    }
}

/// Выполнение программы с двумя аргументами
fn double_argument_processing(param: &str, file_name: &str) {
    match param {
        // Напечатать количество байт
        "-c" | "--bytes" => lib_param::param_c(file_name),
        // Напечатать количество символ
        "-m" | "--chars" => lib_param::param_m(file_name),
        // Напечатать количество новых строк
        "-l" | "--lines" => lib_param::param_l(file_name),
        // Напечатать количество слов
        "-w" | "--words" => lib_param::param_w(file_name),
        // Напечатать максимальную выводимую ширину
        "-L" | "--max-line-length" => lib_param::param_mll(file_name),
        // Вывод справки
        _ => calling_help::title_out(),
    }
}

fn main() {
    // Скипаем имя программы
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    match args.len() {
        2 => {
            // Выполняем программу [param file_name]
            file_verification(&args[1]);
            double_argument_processing(&args[0], &args[1]);
        }
        1 => {
            // Выполнение программы ['-h'|'-v'|file_name]
            single_argument_processing(&args[0]);
        }
        _ => {
            // Вызов справки
            calling_help::title_out();
        }
    }
}