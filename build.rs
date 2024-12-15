use std::env;
use std::path::PathBuf;

fn main() {
    // Укажите путь к директории, где находится libfreertos.a
    let lib_path = PathBuf::from("libs");

    // Укажите путь к заголовочным файлам (если требуется)
    // let include_path = PathBuf::from("libs/include");
    println!("cargo:rerun-if-changed=libs/libfreertos.a");

    // Добавляем путь поиска для линкера
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Связываем статическую библиотеку
    println!("cargo:rustc-link-lib=static=freertos");

    // Если нужны математические функции
    // println!("cargo:rustc-link-lib=c");
    // println!("cargo:rustc-link-lib=m"); 

    // Передаём путь к заголовкам для сторонних библиотек
    // println!("cargo:include={}", include_path.display());
}