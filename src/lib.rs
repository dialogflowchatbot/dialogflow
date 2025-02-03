// #[global_allocator]
// static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

pub(crate) mod ai;
pub(crate) mod db;
pub(crate) mod external;
pub(crate) mod flow;
pub(crate) mod intent;
pub(crate) mod kb;
pub(crate) mod man;
pub(crate) mod result;
pub(crate) mod robot;
// #[cfg(test)]
// pub mod test;
pub(crate) mod variable;
pub mod web;
