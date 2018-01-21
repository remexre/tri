use std::str::FromStr;

use nom::{multispace, IResult};

use commands::List;

impl FromStr for List {
    type Err = ();

    fn from_str(s: &str) -> Result<List, ()> {
        println!("{} -> {:?}", s, parser(s));
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _)
            | IResult::Incomplete(_)
            | IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> List, complete!(alt_complete!(
    do_parse!(
        tag_s!("list")         >>
        multispace             >>
        l: qualifier           >>
        opt!(complete!(tasks)) >>
        ( l )) |
    value!(List::Me, tag_s!("list")))));

named!(qualifier(&str) -> List, alt!(
    value!(List::All,       tag_s!("all"))     |
    value!(List::Everybody, tag_s!("current")) |
    value!(List::Me,        tag_s!("my"))      |
    every_qualifier));

named!(every_qualifier(&str) -> List, do_parse!(
    tag_s!("every")                                 >>
    alt_complete!(tag_s!("body") | tag_s!("one"))   >>
    opt!(alt_complete!(tag_s!("s") | tag_s!("'s"))) >>
    ( List::Everybody )));

named!(tasks(&str) -> &str, recognize!(tuple!(multispace, tag_s!("task"), opt!(tag_s!("s")))));

#[test]
fn list() {
    assert_eq!("list".parse::<List>().unwrap(), List::Me);
    assert_eq!("list all".parse::<List>().unwrap(), List::All);
    assert_eq!("list all tasks".parse::<List>().unwrap(), List::All);
    assert_eq!("list current".parse::<List>().unwrap(), List::Everybody);
    assert_eq!(
        "list current tasks".parse::<List>().unwrap(),
        List::Everybody
    );
    assert_eq!("list everybodys".parse::<List>().unwrap(), List::Everybody);
    assert_eq!(
        "list everybodys tasks".parse::<List>().unwrap(),
        List::Everybody
    );
    assert_eq!("list everybody's".parse::<List>().unwrap(), List::Everybody);
    assert_eq!(
        "list everybody's tasks".parse::<List>().unwrap(),
        List::Everybody
    );
    assert_eq!("list everyone".parse::<List>().unwrap(), List::Everybody);
    assert_eq!(
        "list everyones tasks".parse::<List>().unwrap(),
        List::Everybody
    );
    assert_eq!("list everyone's".parse::<List>().unwrap(), List::Everybody);
    assert_eq!(
        "list everyone's tasks".parse::<List>().unwrap(),
        List::Everybody
    );
    assert_eq!("list my".parse::<List>().unwrap(), List::Me);
    assert_eq!("list my tasks".parse::<List>().unwrap(), List::Me);
}
