//! Collections types
//! 
//! There are such collections as:
//! * Sequences: [`Vector`]
//! 

pub mod vector;
pub mod llist;
pub mod arraystack;
pub mod liststack;
pub mod lqueue;

pub use vector::Vector;
pub use llist::LList;
pub use arraystack::ArrayStack;
pub use liststack::ListStack;
pub use lqueue::LQueue;
