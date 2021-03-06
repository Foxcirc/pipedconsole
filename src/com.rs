
//! This is an internal module, used for communication
//! through a pipe using the [`send`] and [`receive`] function.

#[path = r"com/send.rs"] pub(crate) mod send;
#[path = r"com/receive.rs"] pub(crate) mod receive;
