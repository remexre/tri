use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Write;

use diesel::backend::Backend;
use diesel::expression::AsExpression;
use diesel::expression::bound::Bound;
use diesel::row::Row;
use diesel::types::{FromSql, FromSqlRow, HasSqlType, Integer, IsNull, ToSql, ToSqlOutput};

/// The priority assigned to a task.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

impl AsExpression<Integer> for Priority {
    type Expression = Bound<Integer, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> AsExpression<Integer> for &'a Priority {
    type Expression = Bound<Integer, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl Display for Priority {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.write_str(self.name())
    }
}

impl<DB: Backend<RawValue = [u8]>> FromSql<Integer, DB> for Priority {
    fn from_sql(bytes: Option<&DB::RawValue>) -> Result<Self, Box<Error + Send + Sync>> {
        let i = FromSql::<Integer, DB>::from_sql(bytes)?;
        Priority::from_i32(i).map_err(|e| Box::new(e) as Box<Error + Send + Sync>)
    }
}

impl<DB> FromSqlRow<Integer, DB> for Priority
where
    DB: Backend + HasSqlType<Integer>,
    i32: FromSql<Integer, DB>,
{
    fn build_from_row<T: Row<DB>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
        let i = FromSqlRow::build_from_row(row)?;
        Priority::from_i32(i).map_err(|e| Box::new(e) as Box<Error + Send + Sync>)
    }

    fn fields_needed() -> usize {
        <i32 as FromSqlRow<Integer, DB>>::fields_needed()
    }
}

impl<DB: Backend> ToSql<Integer, DB> for Priority {
    fn to_sql<W: Write>(
        &self,
        out: &mut ToSqlOutput<W, DB>,
    ) -> Result<IsNull, Box<Error + Send + Sync>> {
        let i = *self as i32;
        ToSql::<Integer, DB>::to_sql(&i, out)
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
