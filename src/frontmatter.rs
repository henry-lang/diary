use serde::de::DeserializeOwned;

const FRONTMATTER_SEPERATOR: &str = "---";

#[derive(Debug)]
pub enum ParseError {
    Beginning,
    Seperators,
    Toml(toml::de::Error)
}

pub fn parse<T: DeserializeOwned>(data: &str) -> Result<(T, &str), ParseError> {
    let [start, end] = data
        .match_indices(FRONTMATTER_SEPERATOR)
        .map(|(i, _)| i)
        .next_chunk()
        .map_err(|_| ParseError::Seperators)?;

    if !data.chars().take(start).all(char::is_whitespace) {
        Err(ParseError::Beginning)
    } else {
        let start = start + FRONTMATTER_SEPERATOR.len();
        let content = &data[start..end];
        Ok((toml::from_str(content).map_err(|e| ParseError::Toml(e))?, &data[end + FRONTMATTER_SEPERATOR.len()..]))
    }
}
