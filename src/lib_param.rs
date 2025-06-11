use std::fs;

/// Количество байт
pub fn param_c(file_name: &str) {
    let mut sum_c = 0;
    let mut sum_l = 0;

    let contents = fs::read_to_string(file_name)
        .expect("[ERROR] -- Что-то пошло не так при чтении файла");

    for line in contents.lines() {
        sum_c += line.len();
        sum_l += 1;
    }
    sum_c += sum_l - 1;

    println!("{} {}", sum_c, file_name);
}

/// Количество символов
pub fn param_m(file_name: &str) {
    let mut sum_m = 0;
    let mut sum_l = 0;

    let contents = fs::read_to_string(file_name)
        .expect("[ERROR] -- Что-то пошло не так при чтении файла");

    for line in contents.lines() {
        sum_m += line.chars().count();
        sum_l += 1;
    }
    sum_m += sum_l - 1;

    println!("{} {}", sum_m, file_name);
}

/// Количество новых строк
pub fn param_l(file_name: &str) {
    let mut sum_l = 0;

    let contents = fs::read_to_string(file_name)
        .expect("[ERROR] -- Что-то пошло не так при чтении файла");

    for _ in contents.lines() {
        sum_l += 1;
    }
    sum_l = sum_l - 1;

    println!("{} {}", sum_l, file_name);
}

/// Количество слов
pub fn param_w(file_name: &str) {
    let mut sum_w = 0;

    let contents = fs::read_to_string(file_name)
        .expect("[ERROR] -- Что-то пошло не так при чтении файла");

    for _ in contents.split_whitespace() {
        sum_w += 1;
    }

    println!("{} {}", sum_w, file_name);
}

/// Печатает максимальную выводимую ширину
pub fn param_mll(file_name: &str) {
    let contents = fs::read_to_string(file_name)
        .expect("[ERROR] -- Что-то пошло не так при чтении файла");

    let mut max_line: String = String::new();

    for line in contents.lines() {
        let value_line = line.to_string();
        if value_line.chars().count() > max_line.chars().count() {
            max_line = value_line;
        }
    }

    println!("{} {}", max_line.chars().count(), file_name);
}