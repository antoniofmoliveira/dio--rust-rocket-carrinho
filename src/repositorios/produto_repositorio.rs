use rusqlite::{Connection, Result, params};

use crate::models::produto::Produto;

/// Retorna todos os produtos armazenados no banco de dados.
///
/// # Erros
///
/// Retorna um erro se houver um problema ao executar a query.
///
/// # Exemplo
///
///
pub fn listar(conn: &Connection) -> Result<Vec<Produto>> {
    let mut stmt = conn.prepare("SELECT id,  nome, descricao, imagem, preco FROM produtos")?;
    let produtos_iter = stmt.query_map(params![], |row| {
        Ok(Produto {
            id: row.get(0)?,
            nome: row.get(1)?,
            descricao: row.get(2)?,
            imagem: row.get(3)?,
            preco: row.get(4)?,
        })
    })?;

    produtos_iter.collect()
}

/// Busca um produto pelo id.
///
/// Retorna um Result::Ok contendo o produto, se ele existir.
/// Caso contr rio, retorna um Result::Err com uma mensagem de erro.
pub fn buscar_por_id(conn: &Connection, id: u32) -> Result<Produto> {
    conn.query_row(
        "SELECT id, nome, descricao, imagem, preco FROM produtos WHERE id = ?1",
        params![id],
        |row| {
            Ok(Produto {
                id: row.get(0)?,
                nome: row.get(1)?,
                descricao: row.get(2)?,
                imagem: row.get(3)?,
                preco: row.get(4)?,
            })
        },
    )
}
