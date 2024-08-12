fn main() {
    // Hay 2 formas de hacer una condicion en rust
    // La primera seria la forma convencional
    let x: i32 = 5;
    if x == 5 {
        println!("x es cinco!");
    } else if x == 6 {
        println!("x es seis!");
    } else {
        println!("x no es ni cinco ni seis :(");
    }

    // Y la otra es la condicion ternaria (tiene sintaxis similar a la de javascript)
    let y: i32 = if x == 5 { 10 } else { 15 };

    // Una condicion ternaria no necesariamente tiene que ser en una linea:
    let z: i32 = if x == 5 {
        10
    } else {
        15
    };

    println!("{}, {}, {}", x, y, z);
}