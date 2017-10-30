//! # tri
//!
//! A simple task bot for Slack.

#![deny(missing_docs)]
#![deny(warnings)]
#![recursion_limit = "128"]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate prettytable;
extern crate rayon;
extern crate slack;
extern crate void;

#[macro_use]
mod macros;

pub mod commands;
mod controller;
mod errors;
pub mod models;
mod nlp;
mod priority;
pub mod render;
pub mod schema;

pub use controller::Tri;
pub use errors::*;
pub use priority::Priority;
