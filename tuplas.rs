fn main() {
    let mi_tupla: (&str, i32) = ("Hola mundo", 15);
    println!("{}, {}", mi_tupla.0, mi_tupla.1); // . en una tupla es como hacer [] en una lista
}