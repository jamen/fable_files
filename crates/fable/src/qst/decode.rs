use std::io::{Read,Seek};

use nom::IResult;
use nom::multi::{many1,many0};
use nom::character::complete::line_ending;
use nom::bytes::complete::tag;
use nom::combinator::{opt,all_consuming};

use crate::{Decode,Error};
use crate::script::Call;
use crate::script::decode::decode_call;

use super::Qst;

impl<T: Read + Seek> Decode<Qst> for T {
    fn decode(&mut self) -> Result<Qst, Error> {
        let mut input = Vec::new();
        self.read_to_end(&mut input)?;
        let (_, qst) = all_consuming(Qst::decode_qst)(&input)?;
        Ok(qst)
    }
}

impl Qst {
    pub fn decode_qst(input: &[u8]) -> IResult<&[u8], Qst, Error> {
        let (input, body) = many1(Self::decode_call)(input)?;
        Ok((input, Qst { body: body }))
    }

    pub fn decode_call(input: &[u8]) -> IResult<&[u8], Call, Error> {
        let (input, _) = opt(many0(line_ending))(input)?;
        let (input, call) = decode_call(input)?;
        let (input, _) = tag(";")(input)?;
        let (input, _) = many1(line_ending)(input)?;
        Ok((input, call))
    }
}