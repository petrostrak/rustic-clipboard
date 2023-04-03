pub mod data;
pub mod domain;
pub mod service;
pub mod web;

pub use data::DataError;
pub use domain::clip::field::ShortCode; // Is utilized by the API server and the domain mods
pub use domain::clip::ClipError;
pub use domain::time::Time;
