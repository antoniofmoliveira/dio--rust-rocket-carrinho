use crate::config::cnn::establish_connection;
use crate::models::produto::Produto;
use crate::repositorios::produto_repositorio;

/// Retorna todos os produtos armazenados no banco de dados.
///
/// Se houver um erro ao executar a query, retorna um vetor vazio.
///
/// # Exemplo
///
/// let produtos = produto_servico::listar();
///
pub fn listar() -> Vec<Produto> {
    let conn = establish_connection();
    match produto_repositorio::listar(&conn) {
        Ok(produtos) => produtos,
        Err(err) => {
            eprintln!("Erro ao listar produtos: {}", err);
            Vec::new()
        }
    }
}
