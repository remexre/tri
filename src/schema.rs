//! Diesel-generated bindings to the database schema used by tri.

#![allow(missing_docs)]

no_arg_sql_function!(
    last_insert_rowid,
    ::diesel::types::Integer,
    "Get the id of the last inserted row"
);
infer_schema!("dotenv:DATABASE_URL");
