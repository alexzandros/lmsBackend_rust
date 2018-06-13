extern crate uuid;
extern crate postgres;
extern crate serde_json;
extern crate zmq;

use postgres::{Connection, TlsMode};
use serde_json::{Value};

fn main(){
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REP).unwrap();
    socket.bind("tcp://*:6913").unwrap();
    
    loop{
        let mut peticion = zmq::Message::new().unwrap();
        socket.recv(&mut peticion,0).unwrap();
        println!("recibí: {:?}", peticion.as_str().unwrap());
        socket.send(listar_usuarios(1, 4).as_bytes(),0).unwrap();
    }
}     


fn listar_usuarios(num_pag: i32, elem_por_pagina:i32) -> String{
    let cadena = String::from("postgres://logica_ludica:seguridad_777@localhost/lms_l_l");
    let conexion = Connection::connect(cadena, TlsMode::None).unwrap();
    let respuesta_json:Value = 
        (&conexion.query("SELECT listar_usuarios($1, $2)",
        &[&num_pag,&elem_por_pagina]).unwrap()).get(0).get(0);
    respuesta_json.to_string()
}
