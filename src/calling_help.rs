/// Вывод полной справки о программе
pub fn help_out() {
    let mut text_hp = String::from(env!("CARGO_PKG_NAME"));
    text_hp.push_str(" (v");
    text_hp.push_str(env!("CARGO_PKG_VERSION"));
    text_hp.push_str("), ");
    text_hp.push_str(env!("CARGO_PKG_DESCRIPTION"));
    println!("{}\n", text_hp);
    title_out();
}

/// Краткое пояснение выполнения программы
pub fn title_out() {
    println!("EXAMPLE:\n\tcargo run -- [param] [file] ...");
    println!("\nPARAMETERS:\n\t'-v, --version'  Версия программы\n\t'-h, --help'  Документация программы");
    println!("\t'-l'  Количество строк\n\t'-c'  Количество байт\n\t'-m'  Количество символов\n\t'-L'  Количество символов в самой длинной строке\n\t'-w'  Количество слов");
}