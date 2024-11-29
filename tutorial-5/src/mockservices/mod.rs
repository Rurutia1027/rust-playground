pub mod rust;
pub mod web3;
pub mod world;

pub use rust::RustService;
pub use web3::Web3Service;
pub use world::WorldService;

use axum::response::{IntoResponse, Response};

// pub trait Hello {
//     fn hello(self) -> impl IntoResponse;
// }
