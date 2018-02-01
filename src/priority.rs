use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Write;

use diesel::backend::Backend;
use diesel::deserialize::Queryable;
use diesel::row::Row;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Integer;
use diesel::types::FromSqlRow;

/// The priority assigned to a task.
#[derive(AsExpression, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[sql_type = "Integer"]
#[repr(i32)]
pub enum Priority {
    /// Eventually, whoever asked you will forget that they did.
    Minimal = 0,

    /// It'd be nice if this got done.
    Low = 10,

    /// If there's nothing higher-priority, do this.
    Medium = 20,

    /// This should be done as soon as possible.
    High = 30,

    /// This needs to be done **right now**. Drop everything else (including
    /// homework due more than 24 hours from now).
    Extreme = 40,
}

impl Priority {
    /// Converts the given number to a Priority.
    ///
    /// TODO: Once TryFrom is stable, deprecate/remove this.
    fn from_i32(i: i32) -> Result<Priority, InvalidPriority> {
        match i {
            0 => Ok(Priority::Minimal),
            10 => Ok(Priority::Low),
            20 => Ok(Priority::Medium),
            30 => Ok(Priority::High),
            40 => Ok(Priority::Extreme),
            _ => Err(InvalidPriority(i)),
        }
    }

    /// Gives a name corresponding to the given Priority.
    pub fn name(&self) -> &'static str {
        match *self {
            Priority::Minimal => "minimal",
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Extreme => "extreme",
        }
    }
}

impl Display for Priority {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.write_str(self.name())
    }
}

impl<DB: Backend> FromSqlRow<Integer, DB> for Priority {
    const FIELDS_NEEDED: usize = 1;

    fn build_from_row<T: Row<DB>>(
        row: &mut T,
    ) -> Result<Self, Box<Error + Send + Sync>> {
        unimplemented!()
        //let i = FromSqlRow::build_from_row(row)?;
        //Priority::from_i32(i)
        //.map_err(|e| Box::new(e) as Box<Error + Send + Sync>)
    }
}

impl<DB: Backend> Queryable<Integer, DB> for Priority {
    type Row = Priority;
    fn build(row: Self::Row) -> Self {
        row
    }
}

impl<DB: Backend> ToSql<Integer, DB> for Priority {
    fn to_sql<W: Write>(
        &self,
        out: &mut Output<W, DB>,
    ) -> ::diesel::serialize::Result {
        let n = *self as i32;
        ToSql::<Integer, DB>::to_sql(&n, out)
    }
}

/// The given value is an invalid priority.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InvalidPriority(i32);

impl Display for InvalidPriority {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{} is not a valid Priority.", self.0)
    }
}

impl Error for InvalidPriority {
    fn description(&self) -> &str {
        "An invalid value was read as a Priority."
    }
}
