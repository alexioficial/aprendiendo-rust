fn main() {
    bucle_loop();
    bucle_while();
    bucle_for();
    enumeracion_for();
}

fn bucle_loop() {
    // El bucle loop se usa para hacer iteraciones infinitas pero se puede detener con "break;"
    loop {
        println!("-- Loop --");
        println!("Itera por siempree");
        println!("-- Loop --");
        break;
    }
}

fn bucle_while() {
    // El bucle while es un tipo de bucle que se ejecuta siempre que se cumpla la condicion que contiene
    println!("-- While --");
    let mut completado: bool = false;
    let mut conteo: i32 = 0;
    while !completado {
        println!("{}", conteo);
        conteo += 1;
        completado = conteo >= 5;
    }
    println!("-- While --");
}

fn bucle_for() {
    // El bucle for itera sobre algo (un rango de datos, lista, etc)
    println!("-- For --");
    for x in 0..10 {
        println!("{}", x);
    }

    let mi_lista: [i32; 5] = [1, 2, 3, 4, 5];
    for y in mi_lista {
        println!("{}", y);
    }
    println!("-- For --");
}

fn enumeracion_for() {
    // La enumeracion en un bucle for sirve para saber cuantas veces se ha iterado sobre algo
    // Basicamente, para llevar un conteo
    println!("-- Enumeracion --");
    let nombres: [&str; 3] = ["Marcos", "Pablo", "Roro"];

    // El orden de los iteradores al usar .enumerate es el siguiente: contador, iterador
    for (conteo, nombres) in nombres.iter().enumerate() {
        println!("{}, {}", conteo, nombres);
    }
    println!("-- Enumeracion --");
}