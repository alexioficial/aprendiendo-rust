fn main() {
    let x: i32 = 2;
    let nombre: &str = match x {
        1 => "Pablo",
        2 => "Alexi",
        3 | 4 => "Jose", // 3 o 4
        _ => "Juan" // El _ sirve para representar el caso de "ninguno de los anteriores"
    };
    println!("Nombre: {}", nombre);

    // Match no solamente sirve para asignar valor a una variable, tambien se puede ejecutar codigo normal
    match x {
        1 => println!("Ejemplo 1"),
        2 => println!("Ejemplo 2"),
        3 => println!("Ejemplo 3"),
        4 | 5 => println!("Ejemplo 4 o 5"),
        _ => println!("Ejemplo ninguna de las anteriores")
    };

    // Tambien se puede hacer un match en un rango
    match x {
        1 ..= 5 => println!("x esta entre 1 y 5"),
        6 ..= 10 => println!("x esta entre 6 y 10"),
        _ => println!("x esta fuera de rango")
    };

    // Hay una forma de obtener el valor de la variable a la cual se le esta haciendo el match dentro
    // de un rango: <valor de la variable (puede tener cualquier nombre)> @ <rango>
    match x {
        valor @ 1 ..= 5 => println!("x esta entre 1 y 5. x es {}", valor),
        6 ..= 10 => println!("x esta entre 6 y 10"),
        v @ _ => println!("x fuera de rango, es igual a {}", v)
    };
}