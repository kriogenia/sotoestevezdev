use regex::Regex;

use crate::Token;

#[derive(Debug)]
pub struct LineProcessor<P>
where
    P: Fn(&str) -> Token,
{
    pub(crate) regex: Regex,
    pub(crate) process: fn,
}

fn metadata()
