use crate::models::cliente::Cliente;
use crate::modelviews::produto_view::ProdutoView;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

#[derive(Serialize)]
pub struct PedidoView {
    pub id: u32,
    pub valor_total: f64,
    pub cliente_id: u32,
    pub data: NaiveDateTime,
    pub pago: bool,
    pub cliente: Cliente,
    pub produtos: Vec<ProdutoView>,
}

impl Default for PedidoView {
    /// Creates a default `PedidoView` instance with all fields initialized to zero values or empty collections.
    ///
    /// - `id`: Set to 0, representing no specific identifier.
    /// - `valor_total`: Set to 0.0, indicating no total value.
    /// - `cliente_id`: Set to 0, indicating no specific client.
    /// - `data`: Set to a default `NaiveDateTime` of 0-0-0 00:00:00.
    /// - `pago`: Set to false, indicating unpaid status.
    /// - `cliente`: Initialized with a default `Cliente` instance.
    /// - `produtos`: Initialized as an empty vector, indicating no associated products.
    fn default() -> Self {
        PedidoView {
            id: 0,
            valor_total: 0.0,
            cliente_id: 0,
            data: NaiveDate::from_ymd_opt(0, 0, 0)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            pago: false,
            cliente: Cliente::default(),
            produtos: Vec::new(),
        }
    }
}
