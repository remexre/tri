use std::str::FromStr;

use nom::{IResult, multispace};

use commands::RemindAll;

impl FromStr for RemindAll {
    type Err = ();

    fn from_str(s: &str) -> Result<RemindAll, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _) |
            IResult::Incomplete(_) |
            IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> RemindAll, do_parse!(
    tag_s!("remind") >>
    multispace       >>
    tag_s!("all")    >>
    ( RemindAll )));

#[test]
fn remind_all() {
    assert_eq!("remind all".parse::<RemindAll>().unwrap(), RemindAll);
}
