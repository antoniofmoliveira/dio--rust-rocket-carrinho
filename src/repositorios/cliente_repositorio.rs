use rusqlite::{Connection, Result, params};

use crate::models::cliente::Cliente;

/// Cria um novo cliente no banco de dados.
///
/// # Erros
///
/// Retorna um erro se houver um problema ao executar a query.
///
/// # Exemplo
///
///
pub fn criar(conn: &Connection, nome_str: &str, telefone_str: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO clientes (nome, telefone) VALUES (?1, ?2)",
        params![nome_str, telefone_str],
    )?;
    Ok(())
}

/// Retorna todos os clientes armazenados no banco de dados.
///
/// # Erros
///
/// Retorna um erro se houver um problema ao executar a query.
///
/// # Exemplo
///
///
pub fn listar(conn: &Connection) -> Result<Vec<Cliente>> {
    let mut stmt = conn.prepare("SELECT id, nome, telefone FROM clientes")?;
    let clientes_iter = stmt.query_map(params![], |row| {
        Ok(Cliente {
            id: row.get(0)?,
            nome: row.get(1)?,
            telefone: row.get(2)?,
        })
    })?;

    clientes_iter.collect()
}

/// Busca um cliente pelo id
///
/// Retorna um Result::Ok contendo o cliente, se ele existir.
/// Caso contr rio, retorna um Result::Err com uma mensagem de erro.
pub fn buscar_por_id(conn: &Connection, id: u32) -> Result<Cliente> {
    conn.query_row(
        "SELECT id, nome, telefone FROM clientes WHERE id = ?1",
        params![id],
        |row| {
            Ok(Cliente {
                id: row.get(0)?,
                nome: row.get(1)?,
                telefone: row.get(2)?,
            })
        },
    )
}

/// Updates a client in the database.
///
/// This function executes a SQL UPDATE statement to modify a client in the "clientes" table
/// based on the provided client ID, new name, and new phone number. It returns a `Result`
/// indicating the success or failure of the operation.
///
/// # Arguments
///
/// * `conn` - A reference to the database connection.
/// * `id_cliente` - The ID of the client to be updated.
/// * `novo_nome` - The new name for the client.
/// * `novo_telefone` - The new phone number for the client.
///
/// # Returns
///
/// * `Ok(())` if the update is successful.
/// * `Err(Error)` if there is an error during the execution of the SQL statement.
pub fn atualizar(
    conn: &Connection,
    id_cliente: u32,
    novo_nome: &str,
    novo_telefone: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE clientes SET nome = ?1, telefone = ?2 WHERE id = ?3",
        params![novo_nome, novo_telefone, id_cliente],
    )?;
    Ok(())
}

/// Deletes a client from the database.
///
/// This function executes a SQL DELETE statement to remove a client from the "clientes" table
/// based on the provided client ID. It returns a `Result` indicating the success or failure
/// of the operation.
///
/// # Arguments
///
/// * `conn` - A reference to the database connection.
/// * `id_cliente` - The ID of the client to be deleted.
///
/// # Returns
///
/// * `Ok(())` if the deletion is successful.
/// * `Err(Error)` if there is an error during the execution of the SQL statement.

pub fn excluir(conn: &Connection, id_cliente: u32) -> Result<()> {
    conn.execute("DELETE FROM clientes WHERE id = ?1", params![id_cliente])?;
    Ok(())
}
