use crate::servicos::pedido_servico;
use crate::servicos::produto_servico;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

/// Mostra a lista de produtos para o cliente especificado
///
/// Nesse momento, n o h  verifica o se o cliente existe ou n o.
/// Isso  feito na camada de neg cio, pois s  l  que tem acesso
/// ao banco de dados.
///
/// A lista de produtos  obtida por meio do servi o de produto.
///
/// O template usado  o `compras/index.html.tera`, que  mostrado
/// na pasta `templates`.
///
/// O template recebe dois par metros: `produtos` e `cliente_id`.
/// `produtos`  um vetor de `Produto` que lista todos os produtos
/// cadastrados no sistema.
/// `cliente_id`  o ID do cliente que est  acessando a p gina.
#[get("/comprar/<cliente_id>")]
pub fn index(cliente_id: u32) -> Template {
    let produtos = produto_servico::listar();
    Template::render(
        "compras/index",
        context! { produtos: &produtos, cliente_id: cliente_id },
    )
}

/// Adiciona um produto ao pedido ativo de um cliente
///
/// Dado um `cliente_id` e um `produto_id`, adiciona o produto ao pedido
/// ativo do cliente. Se o produto j  estiver no pedido, ignora a solicita o.
///
/// Se o produto for adicionado ao pedido, redireciona para a p gina do
/// carrinho do cliente.
///
/// Se houver erro ao adicionar o produto ao pedido, redireciona para a
/// p gina inicial.
#[get("/adicionar_ao_pedido/<cliente_id>/<produto_id>")]
pub fn adicionar(cliente_id: u32, produto_id: u32) -> Redirect {
    // Adicionar produto ao pedido
    if pedido_servico::adicionar(cliente_id, produto_id) {
        // Redirecionar para a página do carrinho
        Redirect::to(format!("/carrinho/{}", cliente_id))
    } else {
        // Se houver erro ao adicionar o produto ao pedido, redirecionar para a página inicial
        Redirect::to("/")
    }
}

/// Mostra o carrinho de compras do cliente especificado.
///
/// O carrinho do cliente  obtido por meio do servi o de pedido.
///
/// O template usado  o `compras/carrinho.html.tera`, que  mostrado
/// na pasta `templates`.
///
/// O template recebe dois par metros: `pedido` e `cliente_id`.
/// `pedido`  um `Pedido` que representa o pedido ativo do cliente.
/// `cliente_id`  o ID do cliente que est  acessando a p gina.
#[get("/carrinho/<cliente_id>")]
pub fn carrinho(cliente_id: u32) -> Template {
    let pedido = pedido_servico::ativo(cliente_id);
    Template::render(
        "compras/carrinho",
        context! { pedido: &pedido, cliente_id: cliente_id },
    )
}

/// Exclui o produto especificado do pedido ativo do cliente especificado.
///
/// Ap s excluir o produto do pedido, redireciona para a p gina do carrinho
/// do cliente.
#[post("/pedidos/excluir-item/<cliente_id>/<pedido_id>/<produto_id>")]
pub fn excluir_item(cliente_id: u32, pedido_id: u32, produto_id: u32) -> Redirect {
    pedido_servico::remover_produto(pedido_id, produto_id);
    Redirect::to(format!("/carrinho/{}", cliente_id))
}
