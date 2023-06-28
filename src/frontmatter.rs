use std::ops::Range;

use serde::de::DeserializeOwned;

const FRONTMATTER_SEPERATOR: &str = "---";

#[derive(Debug)]
pub enum ParseError {
    Beginning,
    Seperators,
    Toml(toml::de::Error)
}

// The range that the frontmatter takes up in the given slice
fn range(data: &str) -> Result<Range<usize>, ParseError> {
    let [start, end] = data
        .match_indices(FRONTMATTER_SEPERATOR)
        .map(|(i, _)| i)
        .next_chunk()
        .map_err(|_| ParseError::Seperators)?;

    Ok(start..end)
}

pub fn parse<T: DeserializeOwned>(data: &str) -> Result<(T, &str), ParseError> {
    let range = range(data)?;

    if !data.chars().take(range.start).all(char::is_whitespace) {
        Err(ParseError::Beginning)
    } else {
        let start = range.start + FRONTMATTER_SEPERATOR.len();
        let content = &data[start..range.end];
        Ok((toml::from_str(content).map_err(ParseError::Toml)?, &data[range.end + FRONTMATTER_SEPERATOR.len()..]))
    }
}

pub fn skip(data: &str) -> Result<&str, ParseError> {
    Ok(&data[range(data)?.end + FRONTMATTER_SEPERATOR.len()..])
}
