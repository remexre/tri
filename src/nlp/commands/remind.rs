use std::str::FromStr;

use nom::{multispace, IResult};

use commands::Remind;
use nlp::slack_id;

impl FromStr for Remind {
    type Err = ();

    fn from_str(s: &str) -> Result<Remind, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _)
            | IResult::Incomplete(_)
            | IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> Remind, do_parse!(
    tag_s!("remind") >>
    multispace       >>
    who: who         >>
    ( Remind(who) )));

named!(who(&str) -> Option<String>, alt_complete!(
    map!(slack_id, |s| Some(s.to_string())) |
    value!(None, tag_s!("all"))));

#[test]
fn remind() {
    assert_eq!(
        "remind <@USLACKBOT>".parse::<Remind>().unwrap(),
        Remind(Some("USLACKBOT".to_string()))
    );
    assert_eq!("remind all".parse::<Remind>().unwrap(), Remind(None));
}
