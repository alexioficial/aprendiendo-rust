fn main() {
    let resultado_suma: i32 = sumar(1, 3); // 4
    let funcion_restar: fn(i32, i32) -> i32 = restar; // Esta variable es igual a la funcion restar
    let resultado_resta: i32 = funcion_restar(5, 3);
    println!("El resultado de la suma es {}", resultado_suma);
    println!("El resultado de la resta es {}", resultado_resta);
}

fn sumar(x: i32, y: i32) -> i32 {
    return x + y;
}

fn restar(x: i32, y: i32) -> i32 {
    return x - y;
}