use nom::{
    Finish as _, IResult, Parser,
    bytes::complete::take_while_m_n,
    character::complete::char,
    combinator::{all_consuming, recognize, verify},
    multi::separated_list1,
};

const MAX_LENGTH: usize = 253;
const MAX_LABEL_LENGTH: usize = 63;

pub(super) fn is_valid(value: &str) -> bool {
    all_consuming(host_name).parse(value).finish().is_ok()
}

fn host_name(input: &str) -> IResult<&str, &str> {
    verify(
        recognize(separated_list1(char('.'), label)),
        |value: &str| value.len() <= MAX_LENGTH,
    )
    .parse(input)
}

fn label(input: &str) -> IResult<&str, &str> {
    verify(
        take_while_m_n(1, MAX_LABEL_LENGTH, |character: char| {
            character.is_ascii_alphanumeric() || character == '-'
        }),
        |value: &str| !value.starts_with('-') && !value.ends_with('-'),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(count: usize, length: usize) -> String {
        vec!["a".repeat(length); count].join(".")
    }

    #[test]
    fn valid() {
        for value in [
            "a",
            "A",
            "0",
            "localhost",
            "VaLiD-HoStNaMe",
            "50-name",
            "235235",
            "a--b",
            "example.com",
            "VaLid.HoStNaMe",
            "123.456",
            "postgresql.example.com",
            "x-1.y-2.z-3",
        ] {
            assert!(is_valid(value), "{value}");
        }
    }

    #[test]
    fn invalid() {
        for value in [
            "",
            ".",
            "-",
            "-invalid-name",
            "also-invalid-",
            "asdf@fasd",
            "@asdfl",
            "asd f@",
            "under_score",
            "ümlaut",
            ".invalid",
            "invalid.name.",
            "invalid.-starting.char",
            "invalid.ending-.char",
            "empty..label",
            " leading-space",
            "trailing-space ",
            "nul\0byte",
        ] {
            assert!(!is_valid(value), "{value:?}");
        }
    }

    #[test]
    fn label_length() {
        assert!(is_valid(&labels(1, MAX_LABEL_LENGTH)));
        assert!(!is_valid(&labels(1, MAX_LABEL_LENGTH + 1)));
        assert!(is_valid(&format!("a.{}.b", labels(1, MAX_LABEL_LENGTH))));
        assert!(!is_valid(&format!(
            "a.{}.b",
            labels(1, MAX_LABEL_LENGTH + 1)
        )));
    }

    #[test]
    fn total_length() {
        let base = labels(4, 62);
        assert_eq!(base.len(), 251);

        let max = format!("{base}.a");
        assert_eq!(max.len(), MAX_LENGTH);
        assert!(is_valid(&max));

        let over = format!("{base}.ab");
        assert_eq!(over.len(), MAX_LENGTH + 1);
        assert!(!is_valid(&over));
    }
}
