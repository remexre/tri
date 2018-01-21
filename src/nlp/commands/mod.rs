mod add_task;
mod change_done;
mod change_due_date;
mod change_priority;
mod help;
mod list;
mod remind;

use std::str::FromStr;

use nom::IResult;

use commands::DynamicCommand;
use nlp::commands::add_task::parser as add_task;
use nlp::commands::change_done::parser as change_done;
use nlp::commands::change_due_date::parser as change_due_date;
use nlp::commands::change_priority::parser as change_priority;
use nlp::commands::help::parser as help;
use nlp::commands::list::parser as list;
use nlp::commands::remind::parser as remind;

impl FromStr for DynamicCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<DynamicCommand, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _)
            | IResult::Incomplete(_)
            | IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> DynamicCommand, alt_complete!(
    map!(add_task,        DynamicCommand::AddTask)        |
    map!(change_done,     DynamicCommand::ChangeDone)     |
    map!(change_due_date, DynamicCommand::ChangeDueDate)  |
    map!(change_priority, DynamicCommand::ChangePriority) |
    map!(help,            DynamicCommand::Help)           |
    map!(list,            DynamicCommand::List)           |
    map!(remind,          DynamicCommand::Remind)));

#[test]
fn all_parse() {
    use commands::Command;

    for cmd in DynamicCommand::examples("USLACKBOT") {
        if let Err(()) = cmd.parse::<DynamicCommand>() {
            panic!("Failed to parse {:?}", cmd);
        }
    }
}
