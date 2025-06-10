/// Вывод версии программы
pub fn version_out() {
    let version = String::from(env!("CARGO_PKG_VERSION"));
    println!("Version program my-wc: v{}", version);
}