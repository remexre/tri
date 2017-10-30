mod add_task;
mod help;
mod list;

use std::str::FromStr;

use nom::IResult;

use commands::DynamicCommand;
use nlp::commands::add_task::parser as add_task;
use nlp::commands::help::parser as help;
use nlp::commands::list::parser as list;

impl FromStr for DynamicCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<DynamicCommand, ()> {
        match parser(s) {
            IResult::Done("", p) => Ok(p),
            IResult::Done(_, _) |
            IResult::Incomplete(_) |
            IResult::Error(_) => Err(()),
        }
    }
}

named!(pub parser(&str) -> DynamicCommand, alt_complete!(
    map!(add_task, DynamicCommand::AddTask) |
    map!(help, DynamicCommand::Help) |
    map!(list, DynamicCommand::List)
    ));

#[test]
fn all_parse() {
    use commands::Command;

    for cmd in DynamicCommand::examples("USLACKBOT") {
        if let Err(()) = cmd.parse::<DynamicCommand>() {
            panic!("Failed to parse {:?}", cmd);
        }
    }
}
