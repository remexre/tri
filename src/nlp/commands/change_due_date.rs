use std::str::FromStr;

use nom::{digit, IResult, multispace};

use commands::ChangeDueDate;
use nlp::date;

impl FromStr for ChangeDueDate {
    type Err = ();

    fn from_str(s: &str) -> Result<ChangeDueDate, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _) |
            IResult::Incomplete(_) |
            IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> ChangeDueDate, alt_complete!(set | unset));

named!(set(&str) -> ChangeDueDate, do_parse!(
    tag_s!("set")                      >>
    multispace                         >>
    tag_s!("due")                      >>
    multispace                         >>
    tag_s!("date")                     >>
    multispace                         >>
    for_from                           >>
    multispace                         >>
    id: map_res!(digit, i32::from_str) >>
    multispace                         >>
    tag_s!("to")                       >>
    multispace                         >>
    date: date                         >>
    ( ChangeDueDate(id, Some(date)) )));

named!(unset(&str) -> ChangeDueDate, do_parse!(
    tag_s!("remove")                   >>
    multispace                         >>
    tag_s!("due")                      >>
    multispace                         >>
    tag_s!("date")                     >>
    multispace                         >>
    for_from                           >>
    multispace                         >>
    id: map_res!(digit, i32::from_str) >>
    ( ChangeDueDate(id, None) )));

named!(for_from(&str) -> &str, alt_complete!(
    tag_s!("for") |
    tag_s!("from")));

#[test]
fn change_due_date() {
    use chrono::NaiveDate;

    assert_eq!(
        "set due date for 1 to 2001-02-03"
            .parse::<ChangeDueDate>()
            .unwrap(),
        ChangeDueDate(1, Some(NaiveDate::from_ymd(2001, 2, 3)))
    );
    assert_eq!(
        "remove due date from 2".parse::<ChangeDueDate>().unwrap(),
        ChangeDueDate(2, None)
    );
}
