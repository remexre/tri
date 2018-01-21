use std::str::FromStr;

use nom::IResult;

use commands::Help;

impl FromStr for Help {
    type Err = ();

    fn from_str(s: &str) -> Result<Help, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _)
            | IResult::Incomplete(_)
            | IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> Help, value!(Help, tag_s!("help")));

#[test]
fn help() {
    assert_eq!("help".parse::<Help>().unwrap(), Help);
}
