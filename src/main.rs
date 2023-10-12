enum NombreElementos {
    Entero(i32),
    Flotante(f64),
    Texto(String),
}

fn agregar_elementos(nombre_vector: &mut Vec<NombreElementos>) {
    let elementos = [
        NombreElementos::Entero(42),
        NombreElementos::Entero(33),
        NombreElementos::Flotante(3.14),
        NombreElementos::Flotante(9.81),
        NombreElementos::Texto("Hola, Rust".to_string()),
        NombreElementos::Texto("Hola, yqs".to_string()),
    ];

    nombre_vector.extend(elementos);
}

fn main() {
    let mut mi_nombre_vector1: Vec<NombreElementos> = Vec::new();
    let prueba = 5;

    if prueba == 5 {
        agregar_elementos(&mut mi_nombre_vector1);
    }

    for elemento in &mi_nombre_vector1 {
        match elemento {
            NombreElementos::Entero(valor) => println!("Entero: {}", valor),
            NombreElementos::Flotante(valor) => println!("Flotante: {}", valor),
            NombreElementos::Texto(valor) => println!("Texto: {}", valor),
        }
    }
}
