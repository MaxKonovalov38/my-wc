use std::env;
use std::fs::{File, self};
use std::process;

mod calling_help;
mod calling_version;

/// Выполнение программы
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

    println!(" {} {} {}   {}", sum_l, sum_w, sum_m, file_name);
}

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

fn single_argument_processing(param: &str) {
    match param {
        "-h" => calling_help::help_out(),
        "-v" => calling_version::version_out(),
        _ => {
            println!("param == {}", param);
            file_verification(param);
            working_a_file(param);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    match args.len() {
        2 => println!("args == 2"),
        1 => {
            println!("args == 1");
            single_argument_processing(&args[0]);
        }
        _ => {
            println!("args == 0");
            calling_help::title_out();
        }
    }
}