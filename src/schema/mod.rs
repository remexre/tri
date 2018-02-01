//! Diesel-generated bindings to the database schema used by tri.

#[allow(missing_docs)]
mod generated;

use diesel::sql_types::Integer;

pub use self::generated::*;

no_arg_sql_function!(
    last_insert_rowid,
    Integer,
    "Get the id of the last inserted row"
);
