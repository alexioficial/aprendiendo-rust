fn main() {
    // El tipo de dato del arreglo tiene el siguiente formato:
    // [<tipo de dato de los elementos>; <cantidad de los elementos>]
    let mi_arreglo_1: [i32; 3] = [1, 2, 3];
    println!("La cantidad de elementos del primer arreglo es: {}", mi_arreglo_1.len());

    // Este arreglo tiene 20 elementos que por defectos son (5)
    let mut arreglo_relleno = [5; 20];
    println!("Valor 1 del arreglo relleno antes de ser mutado: {}", arreglo_relleno[0]);
    arreglo_relleno[0] = 10;
    println!("Valor 1 del arreglo relleno despues de ser mutado: {}", arreglo_relleno[0]);
}