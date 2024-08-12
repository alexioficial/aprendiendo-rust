use std::env;
use std::process::Command;

fn main() {
    // Obtener los argumentos pasados al programa
    let args: Vec<String> = env::args().collect();

    // Compilar el archivo Rust pasado como argumento
    Command::new("rustc")
        .arg(&args[1])
        .status()
        .expect("Failed to compile the Rust file");

    // Ejecutar el binario compilado
    let output_file = args[1].split(".rs").next().unwrap();
    Command::new(format!("./{}", output_file))
        .status()
        .expect("Failed to execute the compiled program");
}
