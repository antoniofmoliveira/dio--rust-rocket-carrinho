use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub telefone: String,
}
