use crate::models::cliente::Cliente;
use crate::models::pedido::Pedido;
use crate::modelviews::{pedido_view::PedidoView, produto_view::ProdutoView};
use chrono::NaiveDateTime;
use rusqlite::{Connection, Result, params};
use std::collections::HashMap;

/// Creates a new order in the database.
///
/// This function inserts a new record into the "pedidos" table using the
/// provided total value, client ID, date, and payment status. It returns
/// a `Result` indicating the success or failure of the operation.
///
/// # Arguments
///
/// * `conn` - A reference to the SQLite database connection.
/// * `cliente_id` - The ID of the client associated with the order.
/// * `valor_total` - The total value of the order.
/// * `data` - The date and time of the order.
/// * `pago` - A boolean indicating whether the order is paid.
///
/// # Returns
///
/// * `Ok(())` if the insertion is successful.
/// * `Err(Error)` if there is an error during the execution of the SQL statement.

pub fn criar(
    conn: &Connection,
    cliente_id: u32,
    valor_total: f64,
    data: NaiveDateTime,
    pago: bool,
) -> Result<()> {
    conn.execute(
        "INSERT INTO pedidos (valor_total, cliente_id, data, pago) VALUES (?1, ?2, ?3, ?4)",
        params![valor_total, cliente_id, data, pago],
    )?;
    Ok(())
}

/// Retrieves the active order for a specified client.
///
/// This function queries the "pedidos" table to find an order that is
/// associated with the given `cliente_id` and is marked as unpaid (`pago = 0`).
/// It returns an `Option<Pedido>` indicating the active order, or `None` if
/// there is no active order for the client.
///
/// # Arguments
///
/// * `conn` - A reference to the SQLite database connection.
/// * `cliente_id` - The ID of the client whose active order is to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(Pedido))` if an active order is found.
/// * `Ok(None)` if no active order exists for the client.
/// * `Err(Error)` if there is an error during the execution of the SQL statement.

pub fn ativo(conn: &Connection, cliente_id: u32) -> Result<Option<Pedido>> {
    let mut stmt = conn.prepare(
        "SELECT id, cliente_id, valor_total, data, pago FROM pedidos WHERE cliente_id = ?1 AND pago = 0"
    )?;

    let mut pedido_iter = stmt.query_map(params![cliente_id], |row| {
        Ok(Pedido {
            id: row.get(0)?,
            cliente_id: row.get(1)?,
            valor_total: row.get(2)?,
            data: row.get(3)?,
            pago: row.get(4)?,
        })
    })?;

    let pedido_ativo = pedido_iter.next().transpose()?;

    Ok(pedido_ativo)
}

/// Retrieves the active order for a specified client, including all of its associated products.
///
/// This function queries the "pedidos" table to find an order that is
/// associated with the given `cliente_id` and is marked as unpaid (`pago = 0`).
/// It then queries the "pedido_produtos" and "produtos" tables to retrieve all products
/// associated with the order, and returns a `PedidoView` containing the order's details
/// and all of its associated products. If there is no active order for the client,
/// `None` is returned.
///
/// # Arguments
///
/// * `conn` - A reference to the SQLite database connection.
/// * `cliente_id` - The ID of the client whose active order is to be retrieved.
///
/// # Returns
///
/// * `Ok(Some(PedidoView))` if an active order is found.
/// * `Ok(None)` if no active order exists for the client.
/// * `Err(Error)` if there is an error during the execution of the SQL statement.
pub fn ativo_completo(conn: &Connection, cliente_id: u32) -> Result<Option<PedidoView>> {
    let mut stmt = conn.prepare("
        SELECT 
            p.id AS pedido_id, p.valor_total AS pedido_valor_total, p.data AS pedido_data, p.pago AS pedido_pago,
            c.id AS cliente_id, c.nome AS cliente_nome, c.telefone AS cliente_telefone,
            pr.id AS produto_id, pr.nome AS produto_nome, pr.descricao AS produto_descricao, 
            pr.imagem AS produto_imagem, pr.preco AS produto_preco, pp.quantidade AS produto_quantidade
        FROM pedidos p
        INNER JOIN clientes c ON p.cliente_id = c.id
        INNER JOIN pedido_produtos pp ON p.id = pp.pedido_id
        INNER JOIN produtos pr ON pp.produto_id = pr.id
        WHERE p.cliente_id = ?1 AND p.pago = 0")?;

    let mut rows = stmt.query(params![cliente_id])?;

    let mut produtos_por_pedido: HashMap<u32, PedidoView> = HashMap::new();

    while let Some(row) = rows.next()? {
        let pedido_id: u32 = row.get("pedido_id")?;
        let produto_id: u32 = row.get("produto_id")?;

        let pedido_view = produtos_por_pedido
            .entry(pedido_id)
            .or_insert_with(|| PedidoView {
                id: pedido_id,
                valor_total: row.get("pedido_valor_total").unwrap_or(0.0),
                cliente_id,
                data: row.get("pedido_data").unwrap(),
                pago: row.get("pedido_pago").unwrap(),
                cliente: Cliente {
                    id: cliente_id,
                    nome: row.get("cliente_nome").unwrap(),
                    telefone: row.get("cliente_telefone").unwrap(),
                },
                produtos: Vec::new(),
            });

        pedido_view.produtos.push(ProdutoView {
            id: produto_id,
            nome: row.get("produto_nome").unwrap(),
            descricao: row.get("produto_descricao").unwrap(),
            imagem: row.get("produto_imagem").unwrap(),
            preco: row.get("produto_preco").unwrap(),
            quantidade: row.get("produto_quantidade").unwrap(),
        });
    }

    Ok(produtos_por_pedido.into_values().next())
}
