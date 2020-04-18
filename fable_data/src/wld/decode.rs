use std::io::{Read,Seek};

use crate::nom::{
    IResult,
    many1,
    many_till,
    all_consuming,
};

use crate::{
    Decode,
    Error,
    ScriptField
};

use super::{WldMap, WldRegion, Wld};

impl Decode for Wld {
    fn decode<Source>(source: &mut Source) -> Result<Wld, Error> where
        Source: Read + Seek
    {
        let mut data = Vec::new();
        source.read_to_end(&mut data)?;
        let (_, wld) = all_consuming(Wld::decode_wld)(&data)?;
        Ok(wld)
    }
}

impl Wld {
    pub fn decode_wld(input: &[u8]) -> IResult<&[u8], Wld, Error> {
        let (input, start_initial_quests) = Self::decode_wld_initial_quests(input)?;
        let (input, map_uid_count) = ScriptField::decode_field_named("MapUIDCount")(input)?;
        let (input, thing_manager_uid_count) = ScriptField::decode_field_named("ThingManagerUIDCount")(input)?;
        let (input, maps) = many1(Self::decode_wld_map)(input)?;
        let (input, regions) = many1(Self::decode_wld_region)(input)?;

        Ok(
            (
                input,
                Wld {
                    start_initial_quests: start_initial_quests,
                    map_uid_count: map_uid_count,
                    thing_manager_uid_count: thing_manager_uid_count,
                    maps: maps,
                    regions: regions,
                }
            )
        )
    }

    pub fn decode_wld_initial_quests(input: &[u8]) -> IResult<&[u8], Vec<ScriptField>, Error> {
        let (input, _start) = ScriptField::decode_field_named("START_INITIAL_QUESTS")(input)?;
        let (input, (instrs, _end)) = many_till(ScriptField::decode_field, ScriptField::decode_field_named("END_INITIAL_QUESTS"))(input)?;

        Ok(
            (
                input,
                instrs,
            )
        )
    }

    pub fn decode_wld_map(input: &[u8]) -> IResult<&[u8], WldMap, Error> {
        let (input, new_map) = ScriptField::decode_field_named("NewMap")(input)?;
        let (input, (instrs, _end_instr)) = many_till(ScriptField::decode_field, ScriptField::decode_field_named("EndMap"))(input)?;

        Ok(
            (
                input,
                WldMap {
                    new_map: new_map,
                    instrs: instrs,
                }
            )
        )
    }

    pub fn decode_wld_region(input: &[u8]) -> IResult<&[u8], WldRegion, Error> {
        let (input, new_region) = ScriptField::decode_field_named("NewRegion")(input)?;
        let (input, (instrs, _end)) = many_till(ScriptField::decode_field, ScriptField::decode_field_named("EndRegion"))(input)?;

        Ok(
            (
                input,
                WldRegion {
                    new_region: new_region,
                    instrs: instrs,
                }
            )
        )
    }
}
