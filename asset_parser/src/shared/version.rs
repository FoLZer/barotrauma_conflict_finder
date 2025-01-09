use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub build: Option<u32>,
    pub revision: Option<u32>,
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed_components = s.split('.');
        let major = parsed_components
            .next()
            .ok_or(ParseVersionError::NotEnoughComponents)?
            .parse::<u32>()?;
        let minor = parsed_components
            .next()
            .ok_or(ParseVersionError::NotEnoughComponents)?
            .parse::<u32>()?;
        let build = if let Some(v) = parsed_components.next() {
            Some(v.parse::<u32>()?)
        } else {
            None
        };
        let revision = if let Some(v) = parsed_components.next() {
            Some(v.parse::<u32>()?)
        } else {
            None
        };
        if parsed_components.next().is_some() {
            return Err(ParseVersionError::TooManyComponents);
        }
        Ok(Self {
            major,
            minor,
            build,
            revision,
        })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(build) = self.build else {
            return write!(f, "{}.{}", self.major, self.minor);
        };
        let Some(revision) = self.revision else {
            return write!(f, "{}.{}.{}", self.major, self.minor, build);
        };
        write!(f, "{}.{}.{}.{}", self.major, self.minor, build, revision)
    }
}

#[derive(Debug)]
pub enum ParseVersionError {
    NotEnoughComponents,
    TooManyComponents,
    U32ParsingError(ParseIntError),
}

impl From<ParseIntError> for ParseVersionError {
    fn from(value: ParseIntError) -> Self {
        Self::U32ParsingError(value)
    }
}
