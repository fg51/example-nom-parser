use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::combinator::value;
use nom::{error::ErrorKind as NomErrorKind, IResult};

mod errors;

#[derive(Clone, Debug, PartialEq)]
enum JsonBool {
    True,
    False,
}

fn json_bool(s: &str) -> IResult<&str, JsonBool> {
    alt((
        value(JsonBool::True, tag("true")),
        value(JsonBool::False, tag("false")),
    ))(s)
}

fn parse_sample(s: &str) -> errors::Result<JsonBool> {
    let (s, result) = all_consuming(json_bool)(s).map_err(|nom_err| match nom_err {
        nom::Err::Incomplete(_) => unreachable!(),
        nom::Err::Error(e) => todo!(),
        nom::Err::Failure(e) => todo!(),
    })?;
    return Ok(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_json_bool() {
        assert_eq!(json_bool("false"), Ok(("", JsonBool::False)));
        assert_eq!(json_bool("true"), Ok(("", JsonBool::True)));
        assert!(json_bool("foo").is_err());
    }

    #[test]
    fn test_parse_sample() {
        match parse_sample("false") {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, format!("{:?}", e)),
        }
    }
}
