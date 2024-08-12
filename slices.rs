fn main() {
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4]; // Un slice de a: solo los elementos 1, 2, y 3
    let complete = &a[..]; // Un slice conteniendo todos los elementos de a

    println!("Tamaño de los slices: {} | {}", middle.len(), complete.len());
}