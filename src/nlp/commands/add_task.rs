use std::str::FromStr;

use chrono::NaiveDate;
use nom::{IResult, multispace, rest_s};

use commands::AddTask;
use nlp::{date, slack_id};
use nlp::priority::parser as priority;

impl FromStr for AddTask {
    type Err = ();

    fn from_str(s: &str) -> Result<AddTask, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _) |
            IResult::Incomplete(_) |
            IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> AddTask, do_parse!(
    tag_s!("add")            >>
    multispace               >>
    priority: priority       >>
    multispace               >>
    tag_s!("priority")       >>
    multispace               >>
    tag_s!("task")           >>
    multispace               >>
    due_date: opt!(due_date) >>
    tag_s!("to")             >>
    multispace               >>
    user_slack_id: slack_id  >>
    multispace               >>
    name: rest_s             >>
    ( AddTask {
        due_date,
        name: name.to_string(),
        priority,
        user_slack_id: user_slack_id.to_string(),
    } )));

named!(due_date(&str) -> NaiveDate, do_parse!(
    tag_s!("due") >>
    multispace    >>
    date: date    >>
    multispace    >>
    ( date )));

#[test]
fn add_task() {
    use priority::Priority;

    assert_eq!(
        "add extreme priority task due 2001-02-03 to <@USLACKBOT> buy more snacks"
            .parse::<AddTask>()
            .unwrap(),
        AddTask {
            due_date: Some(NaiveDate::from_ymd(2001, 2, 3)),
            name: "buy more snacks".to_string(),
            priority: Priority::Extreme,
            user_slack_id: "USLACKBOT".to_string(),
        }
    );
    assert_eq!(
        "add minimal priority task to <@USLACKBOT> fix Minnehack registration"
            .parse::<AddTask>()
            .unwrap(),
        AddTask {
            due_date: None,
            name: "fix Minnehack registration".to_string(),
            priority: Priority::Minimal,
            user_slack_id: "USLACKBOT".to_string(),
        }
    );
}
