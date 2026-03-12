/// Models module - Data structures for POS system

pub mod user;
pub mod product;

pub use user::{User, UserRole, AuthState};
#[allow(unused_imports)]
pub use product::{Product, CreateProductRequest, UpdateProductRequest};
