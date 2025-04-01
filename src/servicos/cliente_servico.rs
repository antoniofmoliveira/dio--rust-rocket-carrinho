use crate::config::cnn::establish_connection;
use crate::models::cliente::Cliente;
use crate::repositorios::cliente_repositorio;

/// Retorna todos os clientes armazenados no banco de dados.
///
/// # Erros
///
/// Retorna um erro se houver um problema ao executar a query.
///
/// # Exemplo
///
/// let clientes = cliente_servico::listar();
pub fn listar() -> Vec<Cliente> {
    let conn = establish_connection();
    cliente_repositorio::listar(&conn).unwrap()
}

/// Cria um novo cliente no banco de dados.
///
/// # Erros
///
/// Retorna false se houver um problema ao executar a query.
///
/// # Exemplo
///
/// let cliente = cliente_servico::criar(String::from("Jo o"), String::from("1234567890"));
pub fn criar(nome: String, telefone: String) -> bool {
    let conn = establish_connection();
    cliente_repositorio::criar(&conn, &nome, &telefone).is_ok()
}

/// Alters an existing client in the database.
///
/// This function updates a client in the database based on the provided client ID,
/// new name, and new phone number. It returns a boolean indicating the success
/// or failure of the operation.
///
/// # Arguments
///
/// * `id` - The ID of the client to be updated.
/// * `nome` - The new name for the client.
/// * `telefone` - The new phone number for the client.
///
/// # Returns
///
/// * `true` if the update is successful.
/// * `false` if there is an error during the execution of the SQL statement.
pub fn alterar(id: u32, nome: String, telefone: String) -> bool {
    let conn = establish_connection();
    cliente_repositorio::atualizar(&conn, id, &nome, &telefone).is_ok()
}

/// Deletes a client by ID from the database.
///
/// Establishes a connection to the database and attempts to delete the client
/// with the specified ID using the `cliente_repositorio::excluir` function.
/// Returns a boolean indicating whether the operation was successful.
///
/// # Arguments
///
/// * `id` - The ID of the client to be deleted.
///
/// # Returns
///
/// * `true` if the deletion is successful.
/// * `false` if there is an error during the execution of the SQL statement.
pub fn excluir_por_id(id: u32) -> bool {
    let conn = establish_connection();
    cliente_repositorio::excluir(&conn, id).is_ok()
}

/// Busca um cliente pelo id.
///
/// Retorna um Result::Ok contendo o cliente, se ele existir.
/// Caso contr rio, retorna um Result::Err com uma mensagem de erro.
/// Se o cliente n o existir, retorna um Cliente padr o.
pub fn buscar_por_id(id: u32) -> Cliente {
    let conn = establish_connection();
    cliente_repositorio::buscar_por_id(&conn, id).unwrap_or_else(|_| Cliente {
        id,
        nome: "Cliente não encontrado".to_string(),
        telefone: "".to_string(),
    })
}
