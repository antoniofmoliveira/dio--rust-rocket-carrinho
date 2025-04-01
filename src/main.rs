#[macro_use] extern crate rocket;

mod models;
mod servicos;
mod dtos;
mod repositorios;
mod config;
mod modelviews;

mod controllers {
    pub mod home_controller;
    pub mod clientes_controller;
    pub mod compras_controller;
}

use rocket_dyn_templates::Template;
use controllers::{ home_controller, clientes_controller, compras_controller };
use rocket::fs::{FileServer, relative};

/// Configures and launches the Rocket application:
/// - Mounts routes for home, compras, and clientes controllers.
/// - Serves static files from the "/static" directory.
/// - Attaches the template fairing for dynamic template rendering.

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        home_controller::index,

        compras_controller::index,
        compras_controller::adicionar,
        compras_controller::carrinho,
        compras_controller::excluir_item,

        clientes_controller::index,
        clientes_controller::novo,
        clientes_controller::criar,
        clientes_controller::editar,
        clientes_controller::alterar,
        clientes_controller::excluir,
    ])
    .mount("/static", FileServer::from(relative!("static")))
    .attach(Template::fairing())
}
