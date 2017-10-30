//! Parsers for the various types.
//!
//! (In fact, this is just a "normal" parser, rather than any NLP-specific
//! thing. This should be changed at some point.)

mod commands;
mod priority;

use std::str::FromStr;

use chrono::NaiveDate;
use nom::digit;

named!(date(&str) -> NaiveDate, do_parse!(
    year: map_res!(digit, i32::from_str)  >>
    tag_s!("-")                           >>
    month: map_res!(digit, u32::from_str) >>
    tag_s!("-")                           >>
    day: map_res!(digit, u32::from_str)   >>
    ( NaiveDate::from_ymd(year, month, day) )));

named!(slack_id(&str) -> &str, do_parse!(
    tag_s!("<@")                       >>
    id: take_until_and_consume_s!(">") >>
    ( id )));
