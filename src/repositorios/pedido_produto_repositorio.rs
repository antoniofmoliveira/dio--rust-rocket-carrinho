use rusqlite::{Connection, Result, params};

/// Creates or updates the quantity of a product in a specific order.
///
/// This function checks if a specified product is already part of an order.
/// If the product exists, its quantity is incremented by one. If not, the product
/// is added to the order with a quantity of one. After updating the product quantity,
/// the total value of the order is recalculated based on the prices of the products
/// and their quantities.
///
/// # Arguments
///
/// * `conn` - A reference to the SQLite database connection.
/// * `pedido_id` - The ID of the order to which the product belongs.
/// * `produto_id` - The ID of the product whose quantity is to be checked or updated.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the operation is successful, or an error if it fails.
pub fn cria_se_nao_existir_ou_atualiza_quantidade(
    conn: &Connection,
    pedido_id: u32,
    produto_id: u32,
) -> Result<()> {
    let query = "SELECT quantidade FROM pedido_produtos WHERE pedido_id = ?1 AND produto_id = ?2";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(params![pedido_id, produto_id])?;

    let mut produto_existe = false;
    let quantidade: i32 = if let Some(row) = rows.next()? {
        produto_existe = true;
        row.get(0)?
    } else {
        0
    };

    if produto_existe {
        let nova_quantidade = quantidade + 1;
        conn.execute(
            "UPDATE pedido_produtos SET quantidade = ?1 WHERE pedido_id = ?2 AND produto_id = ?3",
            params![nova_quantidade, pedido_id, produto_id],
        )?;
    } else {
        conn.execute(
            "INSERT INTO pedido_produtos (pedido_id, produto_id, quantidade) VALUES (?1, ?2, 1)",
            params![pedido_id, produto_id],
        )?;
    }

    let total_query = "SELECT SUM(p.preco * pp.quantidade) FROM produtos p INNER JOIN pedido_produtos pp ON p.id = pp.produto_id WHERE pp.pedido_id = ?1";
    let total: f64 = conn.query_row(total_query, params![pedido_id], |row| row.get(0))?;

    conn.execute(
        "UPDATE pedidos SET valor_total = ?1 WHERE id = ?2",
        params![total, pedido_id],
    )?;

    Ok(())
}

/// Remove a quantidade de um produto em um pedido.
///
/// Verifica se o produto existe no pedido. Se sim, decrementa a quantidade
/// do produto. Se a quantidade for 1, exclui o registro. Em seguida,
/// atualiza o valor total do pedido.
///
/// # Arguments
///
/// * `conn` - A refer ncia para a conex o do banco de dados SQLite.
/// * `pedido_id` - O ID do pedido.
/// * `produto_id` - O ID do produto.
///
/// # Returns
///
/// * `Result<()>` - Retorna `Ok(())` se a opera o for bem sucedida, ou
/// um erro se falhar.
pub fn remove_quantidade_por_id(conn: &Connection, pedido_id: u32, produto_id: u32) -> Result<()> {
    // Verifica se o pedido_produto existe
    let query = "SELECT quantidade FROM pedido_produtos WHERE pedido_id = ?1 AND produto_id = ?2";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(params![pedido_id, produto_id])?;

    if let Some(row) = rows.next()? {
        let quantidade: i32 = row.get(0)?;

        if quantidade > 1 {
            let nova_quantidade = quantidade - 1;
            conn.execute(
                "UPDATE pedido_produtos SET quantidade = ?1 WHERE pedido_id = ?2 AND produto_id = ?3",
                params![nova_quantidade, pedido_id, produto_id],
            )?;
        } else {
            conn.execute(
                "DELETE FROM pedido_produtos WHERE pedido_id = ?1 AND produto_id = ?2",
                params![pedido_id, produto_id],
            )?;
        }

        let total_query = "SELECT SUM(p.preco * pp.quantidade) FROM produtos p INNER JOIN pedido_produtos pp ON p.id = pp.produto_id WHERE pp.pedido_id = ?1";
        let total: f64 = conn.query_row(total_query, params![pedido_id], |row| row.get(0))?;

        conn.execute(
            "UPDATE pedidos SET valor_total = ?1 WHERE id = ?2",
            params![total, pedido_id],
        )?;
    }

    Ok(())
}
