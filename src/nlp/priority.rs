use std::str::FromStr;

use nom::IResult;

use priority::Priority;

impl FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Priority, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _) |
            IResult::Incomplete(_) |
            IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> Priority, alt_complete!(
    value!(Priority::Minimal, tag_s!("minimal")) |
    value!(Priority::Low,     tag_s!("low"))     |
    value!(Priority::Medium,  tag_s!("medium"))  |
    value!(Priority::High,    tag_s!("high"))    |
    value!(Priority::Extreme, tag_s!("extreme"))));

#[test]
fn priority() {
    assert_eq!("minimal".parse::<Priority>().unwrap(), Priority::Minimal);
    assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
    assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);
    assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
    assert_eq!("extreme".parse::<Priority>().unwrap(), Priority::Extreme);
}
