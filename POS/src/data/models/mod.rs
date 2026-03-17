pub mod product;
/// Models module - Data structures for POS system
pub mod user;

#[allow(unused_imports)]
pub use product::{CreateProductRequest, Product, UpdateProductRequest};
#[allow(unused_imports)]
pub use user::{AuthState, User, UserRole};
