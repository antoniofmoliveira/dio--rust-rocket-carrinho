use crate::config::cnn::establish_connection;
use crate::modelviews::pedido_view::PedidoView;
use crate::repositorios::{pedido_produto_repositorio, pedido_repositorio, produto_repositorio};

/// Adiciona um produto ao pedido ativo de um cliente.
///
/// Dado um `cliente_id` e um `produto_id`, adiciona o produto ao pedido
/// ativo do cliente. Se o produto j  estiver no pedido, ignora a solicita o.
/// Se o pedido n o existir, cria um novo com o produto em quest o.
///
/// Retorna `true` se a opera o for bem sucedida, ou `false` caso contr rio.
pub fn adicionar(cliente_id: u32, produto_id: u32) -> bool {
    let conn = establish_connection();

    let pedido = match pedido_repositorio::ativo(&conn, cliente_id) {
        Ok(Some(pedido)) => pedido,
        Ok(None) => {
            if let Err(err) = pedido_repositorio::criar(
                &conn,
                cliente_id,
                0.0,
                chrono::Local::now().naive_local(),
                false,
            ) {
                eprintln!("Erro ao criar pedido: {}", err);
                return false;
            }
            return adicionar(cliente_id, produto_id);
        }
        Err(err) => {
            eprintln!("Erro ao buscar pedido ativo: {}", err);
            return false;
        }
    };

    let produto = match produto_repositorio::buscar_por_id(&conn, produto_id) {
        Ok(produto) => produto,
        Err(err) => {
            eprintln!("Erro ao buscar produto: {}", err);
            return false;
        }
    };

    if let Err(err) = pedido_produto_repositorio::cria_se_nao_existir_ou_atualiza_quantidade(
        &conn, pedido.id, produto.id,
    ) {
        eprintln!("Erro ao adicionar produto ao pedido: {}", err);
        return false;
    }

    true
}

    /// Retorna o pedido ativo do cliente especificado.
    ///
    /// Caso o cliente n o tenha um pedido ativo, retorna um `PedidoView` vazio.
    /// Caso haja um erro ao buscar o pedido ativo, tamb m   retorna um `PedidoView` vazio
    /// e imprime o erro na sa da padr o.
pub fn ativo(cliente_id: u32) -> PedidoView {
    let conn = establish_connection();

    match pedido_repositorio::ativo_completo(&conn, cliente_id) {
        Ok(Some(pedido_view)) => pedido_view,
        Ok(None) => PedidoView::default(), // Retorna diretamente um PedidoView vazio se não houver pedido ativo
        Err(err) => {
            eprintln!("Erro ao buscar pedido ativo: {}", err);
            PedidoView::default() // Retorna PedidoView vazio também no caso de erro
        }
    }
}

    /// Remove um produto de um pedido.
    ///
    /// Se o produto existir no pedido, remove uma unidade do produto do pedido.
    /// Se a quantidade do produto for 1, remove o registro do produto do pedido.
    /// Se houver erro durante a opera o, retorna false e imprime o erro na sa da padr o.
    ///
    /// # Arguments
    ///
    /// * `pedido_id` - O ID do pedido.
    /// * `produto_id` - O ID do produto.
    ///
    /// # Returns
    ///
    /// * `true` se a opera o for bem sucedida, ou `false` se falhar.
pub fn remover_produto(pedido_id: u32, produto_id: u32) -> bool {
    let conn = establish_connection();

    match pedido_produto_repositorio::remove_quantidade_por_id(&conn, pedido_id, produto_id) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Erro ao remover produto: {}", e);
            false
        }
    }
}
