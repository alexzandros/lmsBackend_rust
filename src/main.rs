extern crate uuid;
extern crate postgres;
#[macro_use]
extern crate serde_json;
extern crate zmq;

use postgres::{Connection, TlsMode};
use serde_json::{Value};

fn main(){
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REP).unwrap();
    socket.bind("tcp://*:6913").unwrap();
    
    loop{
        let peticion = socket.recv_string(0).unwrap();
        println!("recibí: {:?}", peticion);
        let respuesta = match peticion{
                Ok(cadena) => enrutar(cadena),
                _ => String::from("Error")
            };
        socket.send(respuesta.as_bytes(), 0);
    }
}     

fn enrutar(peticion:String) -> String{
    let cadenas:Vec<&str> = peticion.split(" ").collect();
    let verbo = cadenas[0];
    match verbo{
        "listar_usuarios" =>
            listar_usuarios(cadenas[1].parse().unwrap(),
                cadenas[2].parse().unwrap()).to_string(),
        "insertar_usuario" =>{ 
            insertar_usuario(&peticion[16..]).to_string()},
        _ => String::from("Método no definido")
    }
}

fn listar_usuarios(num_pag: i32, elem_por_pagina:i32) -> Value{
    let cadena = String::from("postgres://logica_ludica:seguridad_777@localhost/lms_l_l");
    let conexion = Connection::connect(cadena, TlsMode::None).unwrap();
    let respuesta_json:Value = 
        (&conexion.query("SELECT listar_usuarios($1, $2)",
        &[&num_pag,&elem_por_pagina]).unwrap()).get(0).get(0);
    respuesta_json
}

fn insertar_usuario(cadena_usuario: &str) -> Value{
    let usuario:Value = serde_json::from_str(cadena_usuario).unwrap();
    let cadena_conexion =
        String::from("postgres://logica_ludica:seguridad_777@localhost/lms_l_l");
    let conexion = Connection::connect(cadena_conexion, TlsMode::None).unwrap();
    let respuesta:uuid::Uuid = 
        conexion.query("SELECT insert_usuario($1)", &[&usuario])
        .unwrap().get(0).get(0);
    json!(respuesta)

}
