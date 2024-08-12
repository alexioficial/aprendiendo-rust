struct Carro {
    marca: String,
    modelo: String,
    precio: i32,
}

impl Carro {
    fn new(marca: &str, modelo: &str, precio: i32) -> Carro {
        Carro { 
            marca: marca.to_string(), 
            modelo: modelo.to_string(), 
            precio 
        }
    }

    fn modificar_precio(&mut self, nuevo_precio: i32) {
        self.precio = nuevo_precio;
    }
}

struct Concesionario {
    nombre: String,
    carros: Vec<Carro>,
}

impl Concesionario {
    fn new(nombre: &str) -> Concesionario {
        Concesionario { 
            nombre: nombre.to_string(), 
            carros: Vec::new(), 
        }
    }

    fn agregar_carro(&mut self, carro: Carro) {
        self.carros.push(carro);
    }

    fn lista_carros(&self) {
        for carro in &self.carros {
            println!("{}, {}. Precio: {}", carro.marca, carro.modelo, carro.precio);
        }
    }
}

fn main() {
    let mut supra = Carro::new("Toyota", "Supra mk4", 90000);
    supra.modificar_precio(100000);
    let skyline = Carro::new("Nissan", "Skyline GTR", 120000);

    let mut concesionario = Concesionario::new("Auto Plaza");

    concesionario.agregar_carro(supra);
    concesionario.agregar_carro(skyline);

    println!("{}", concesionario.nombre);
    concesionario.lista_carros();
}
