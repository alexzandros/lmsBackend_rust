extern crate uuid;
extern crate postgres;
#[macro_use]
extern crate serde_json;
extern crate zmq;

use postgres::{Connection, TlsMode};
use serde_json::{Value};
use std::result::Result;

fn main(){
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REP).expect("No es posible crear el zócalo");
    socket.bind("tcp://*:6913").expect("Puerto no disponible");
    
    loop{
        let peticion = socket.recv_string(0).expect("No es posible recibir peticiones del cliente");
        println!("recibí: {:?}", peticion);
        let respuesta = match peticion{
                Ok(cadena) => enrutar(cadena),
                _ => Err(String::from("Error"))
            };
        let cadena = match respuesta{
            Ok(cadena) => cadena,
            Err(ref cadena) => {
                let mut cadena_enviar = String::from("Error - ");
                cadena_enviar.push_str(cadena);
                json!(cadena_enviar).to_string()
            }
        };
        socket.send(cadena.as_bytes(),0).expect("No es posible enviar respuesta al cliente");
    }
}

fn enrutar(peticion:String) -> Result<String,String>{
    let cadenas:Vec<&str> = peticion.split(" ").collect();
    let verbo = cadenas[0];
    let resultado = match verbo{
        "listar_usuarios" =>{
            let num_pag = match cadenas[1].parse(){
                Ok(num) => num,
                Err(_) => 1
            };
            let item_por_pag = match cadenas[2].parse(){
                Ok(num) => num,
                Err(_) => 10
            };
            listar_usuarios(num_pag, item_por_pag)  
        },
        "insertar_usuario" => insertar_usuario(&peticion[16..]),
        "loguear_usuario" => loguear_usuario(&peticion[15..]),
        _ => Err(String::from("Método no definido"))
    };
    match resultado{
        Ok(valor_json) => Ok(valor_json.to_string()),
        Err(error)  => Err( error)
    }
}

fn listar_usuarios(num_pag: i32, elem_por_pagina:i32) -> Result<Value, String>{
    let cadena_conexion = String::from("postgres://logica_ludica:seguridad_777@localhost/lms_l_l");
    let conexion = match Connection::connect(cadena_conexion, TlsMode::None){
        Ok(conn) => conn,
        Err(_) => return Err(String::from("No es posible conectar a la Base de datos"))
    };
    let respuesta_bd = match conexion.query("SELECT listar_usuarios($1, $2)",
        &[&num_pag,&elem_por_pagina]){
            Ok(r) => r,
            Err(error_bd) => return Err(error_bd.to_string())

        };
    let respuesta_json:Value = respuesta_bd.get(0).get(0);
    Ok(respuesta_json)
}

fn loguear_usuario(cadena_usuario: &str) -> Result<Value, String>{
    let datos_usuario:Value = match serde_json::from_str(cadena_usuario){
        Ok(valor) => valor,
        Err(_) => return Err(String::from("Datos en JSON mal formateados"))
    };
    let cadena_conexion =
        String::from("postgres://logica_ludica:seguridad_777@localhost/lms_l_l");
    let conexion = match Connection::connect(cadena_conexion, TlsMode::None){
        Ok(conn) => conn,
        Err(_) => return Err(String::from("No es posible conectar a la Base de datos"))
    };
    let respuesta:String = match conexion.query("SELECT loguear_usuario($1)",
        &[&datos_usuario]){
            Ok(valor) => valor.get(0).get(0),
            Err(error_bd) => return Err(error_bd.to_string())
    };
    Ok(json!(respuesta))
}

fn insertar_usuario(cadena_usuario: &str) -> Result<Value,String>{
    let usuario:Value = match serde_json::from_str(cadena_usuario){
        Ok(valor) => valor,
        Err(_) => return Err(String::from("Datos en JSON mal formateados"))
    };
    let cadena_conexion =
        String::from("postgres://logica_ludica:seguridad_777@localhost/lms_l_l");
    let conexion = match Connection::connect(cadena_conexion, TlsMode::None){
        Ok(conn) => conn,
        Err(_) => return Err(String::from("No es posible conectar a la Base de datos"))
    };
    let respuesta:uuid::Uuid = match conexion.query("SELECT insert_usuario($1)", &[&usuario]){
        Ok(valor) => valor.get(0).get(0),
        Err(error_bd) => return Err(error_bd.to_string())
    };  
    Ok(json!(respuesta))
}
