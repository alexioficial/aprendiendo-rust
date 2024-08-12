fn main() {
    let x: i32 = 5; // No es mutable
    let mut y: &str = "Hola mundo"; // Es mutable
    println!("Valor de y actual: {}", y);
    y = "Adios mundo";
    println!("Valor de y luego de mutar: {}", y);
    let (a, b) = (5, 10);
    println!("El valor de x es {}", x);
    println!("El valor de a es {}", a);
    println!("El valor de b es {}", b);
}