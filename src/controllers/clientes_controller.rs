use crate::servicos::cliente_servico;
use rocket_dyn_templates::{Template, context};

use crate::dtos::cliente_dto::ClienteDto;
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

/// Renders the clients index page.
///
/// This function handles GET requests to the "/clientes" endpoint.
/// It retrieves a list of all clients from the `cliente_servico` and
/// renders the "clientes/index" template with the list of clients in the context.

#[get("/clientes")]
pub fn index() -> Template {
    let clientes = cliente_servico::listar();
    Template::render("clientes/index", context! { clientes: &clientes })
}

/// Renders the new client page.
///
/// This function handles GET requests to the "/clientes/novo" endpoint.
/// It renders the "clientes/novo" template with an error message in the context if
/// there was a flash message.
///
/// This function is invoked when the user navigates to the "/clientes/novo" path.
#[get("/clientes/novo")]
pub fn novo(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("clientes/novo", context! { erro: erro_flash(flash) })
}

/// Creates a new client and handles the client creation request.
///
/// This function handles POST requests to the "/clientes/criar" endpoint.
/// It receives a form submission containing client data, attempts to create
/// a new client using the `cliente_servico::criar` function, and redirects
/// the user based on the outcome. On success, it redirects to the "/clientes"
/// page. On failure, it redirects back to the "/clientes/novo" page with an
/// error message.
///
/// # Arguments
///
/// * `cliente_dto_form` - A form containing the client data to be created.
///
/// # Returns
///
/// * `Ok(Redirect)` to the clients index page if the creation is successful.
/// * `Err(Box<Flash<Redirect>>)` with an error message redirecting to the new
///   client page if the creation fails.

#[post("/clientes/criar", data = "<cliente_dto_form>")]
pub fn criar(
    cliente_dto_form: Form<ClienteDto>,
) -> Result<Redirect, Box<rocket::response::Flash<rocket::response::Redirect>>> {
    let cliente_dto = cliente_dto_form.into_inner();

    if cliente_servico::criar(cliente_dto.nome, cliente_dto.telefone) {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Box::new(Flash::error(
            Redirect::to("/clientes/novo"),
            "Erro ao cadastrar cliente",
        )))
    }
}

/// Renders the edit client page.
///
/// This function handles GET requests to the "/clientes/<id>/editar" endpoint.
/// It retrieves a client by its ID using the `cliente_servico::buscar_por_id` function,
/// and renders the "clientes/editar" template with the client object and an error
/// message in the context if there was a flash message.
///
/// This function is invoked when the user navigates to the "/clientes/<id>/editar"
/// path.
#[get("/clientes/<id>/editar")]
pub fn editar(id: u32, flash: Option<FlashMessage<'_>>) -> Template {
    let cliente = cliente_servico::buscar_por_id(id);
    Template::render(
        "clientes/editar",
        context! {
            cliente: &cliente,
            erro: erro_flash(flash)
        },
    )
}

/// Alters an existing client and handles the client alteration request.
///
/// This function handles POST requests to the "/clientes/<id>/alterar" endpoint.
/// It receives a form submission containing client data, attempts to alter the
/// client using the `cliente_servico::alterar` function, and redirects the user
/// based on the outcome. On success, it redirects to the "/clientes" page. On
/// failure, it redirects back to the "/clientes/<id>/editar" page with an error
/// message.
///
/// # Arguments
///
/// * `id` - The ID of the client to be altered.
/// * `cliente_dto_form` - A form containing the client data to be altered.
///
/// # Returns
///
/// * `Ok(Redirect)` to the clients index page if the alteration is successful.
/// * `Err(Box<Flash<Redirect>>)` with an error message redirecting to the edit
///   client page if the alteration fails.
#[post("/clientes/<id>/alterar", data = "<cliente_dto_form>")]
pub fn alterar(
    id: u32,
    cliente_dto_form: Form<ClienteDto>,
) -> Result<Redirect, Box<rocket::response::Flash<rocket::response::Redirect>>> {
    let cliente_dto = cliente_dto_form.into_inner();

    if cliente_servico::alterar(id, cliente_dto.nome, cliente_dto.telefone) {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Box::new(Flash::error(
            Redirect::to(format!("/clientes/{}/editar", id)),
            "Erro ao alterar cliente",
        )))
    }
}

/// Excludes a client and handles the client exclusion request.
///
/// This function handles GET requests to the "/clientes/<id>/excluir" endpoint.
/// It attempts to exclude the client using the `cliente_servico::excluir_por_id`
/// function and redirects the user based on the outcome. On success, it redirects
/// to the "/clientes" page. On failure, it redirects back to the "/clientes/<id>/editar"
/// page with an error message.
///
/// # Arguments
///
/// * `id` - The ID of the client to be excluded.
///
/// # Returns
///
/// * `Ok(Redirect)` to the clients index page if the exclusion is successful.
/// * `Err(Box<Flash<Redirect>>)` with an error message redirecting to the edit
///   client page if the exclusion fails.
#[get("/clientes/<id>/excluir")]
pub fn excluir(
    id: u32,
) -> Result<Redirect, Box<rocket::response::Flash<rocket::response::Redirect>>> {
    if cliente_servico::excluir_por_id(id) {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Box::new(Flash::error(
            Redirect::to(format!("/clientes/{}/editar", id)),
            "Erro ao excluir cliente",
        )))
    }
}

/// Retrieves the error message from a flash message if the flash message is an
/// error message. Otherwise, returns an empty string.
///
/// # Arguments
///
/// * `flash` - An optional `FlashMessage` containing the error message.
///
/// # Returns
///
/// * An empty string if the `FlashMessage` is `None` or if the `FlashMessage` is
///   not an error message.
/// * The error message as a string if the `FlashMessage` is an error message.
fn erro_flash(flash: Option<FlashMessage<'_>>) -> String {
    let mut erro = "".to_string();
    if let Some(msg) = flash {
        if msg.kind() == "error" {
            erro = msg.message().to_string();
        }
    }

    erro
}
