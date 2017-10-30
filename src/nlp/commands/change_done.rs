use std::str::FromStr;

use nom::{digit, IResult, multispace};

use commands::ChangeDone;

impl FromStr for ChangeDone {
    type Err = ();

    fn from_str(s: &str) -> Result<ChangeDone, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _) |
            IResult::Incomplete(_) |
            IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> ChangeDone, alt_complete!(short | long));

named!(short(&str) -> ChangeDone, do_parse!(
    tag_s!("done")                     >>
    multispace                         >>
    id: map_res!(digit, i32::from_str) >>
    ( ChangeDone(id, true) )));

named!(long(&str) -> ChangeDone, do_parse!(
    state: set                         >>
    multispace                         >>
    tag_s!("done")                     >>
    multispace                         >>
    id: map_res!(digit, i32::from_str) >>
    ( ChangeDone(id, state) )));

named!(set(&str) -> bool, alt_complete!(
    value!(true,  tag_s!("set")) |
    value!(false, tag_s!("unset"))));

#[test]
fn change_done() {
    assert_eq!("done 1".parse::<ChangeDone>().unwrap(), ChangeDone(1, true));
    assert_eq!(
        "set done 2".parse::<ChangeDone>().unwrap(),
        ChangeDone(2, true)
    );
    assert_eq!(
        "unset done 3".parse::<ChangeDone>().unwrap(),
        ChangeDone(3, false)
    );
}
