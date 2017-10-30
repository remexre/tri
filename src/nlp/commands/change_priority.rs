use std::str::FromStr;

use nom::{digit, IResult, multispace};

use commands::ChangePriority;
use nlp::priority::parser as priority;

impl FromStr for ChangePriority {
    type Err = ();

    fn from_str(s: &str) -> Result<ChangePriority, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _) |
            IResult::Incomplete(_) |
            IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> ChangePriority, do_parse!(
    tag_s!("set")                      >>
    multispace                         >>
    tag_s!("priority")                 >>
    multispace                         >>
    tag_s!("for")                      >>
    multispace                         >>
    id: map_res!(digit, i32::from_str) >>
    multispace                         >>
    tag_s!("to")                       >>
    multispace                         >>
    priority: priority                 >>
    ( ChangePriority(id, priority) )));

#[test]
fn change_priority() {
    use priority::Priority;

    assert_eq!(
        "set priority for 1 to high"
            .parse::<ChangePriority>()
            .unwrap(),
        ChangePriority(1, Priority::High)
    );
    assert_eq!(
        "set priority for 2 to low"
            .parse::<ChangePriority>()
            .unwrap(),
        ChangePriority(2, Priority::Low)
    );
    assert_eq!(
        "set priority for 3 to medium"
            .parse::<ChangePriority>()
            .unwrap(),
        ChangePriority(3, Priority::Medium)
    );
}
