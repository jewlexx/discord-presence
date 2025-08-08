/// [`DisplayType`] enum
///
/// Lists all activity types currently supported by Discord.
///
/// This may change in future if Discord adds support for more types,
/// or removes support for some.
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DisplayType {
    /// Playing a game
    Name = 0,
    /// Listening to...
    State = 1,
    /// Watching...
    Details = 2,
}

impl<'de> serde::Deserialize<'de> for DisplayType {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const NAME: u8 = DisplayType::Name as u8;
        const STATE: u8 = DisplayType::State as u8;
        const DETAILS: u8 = DisplayType::Details as u8;

        match u8::deserialize(deserializer)? {
            NAME => Result::Ok(DisplayType::Name),
            STATE => Result::Ok(DisplayType::State),
            DETAILS => Result::Ok(DisplayType::Details),
            other => Result::Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(u64::from(other)),
                &(&format!("one of: {NAME}, {STATE}, {DETAILS}") as &str),
            )),
        }
    }
}

impl serde::Serialize for DisplayType {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match *self {
            DisplayType::Name => DisplayType::Name as u8,
            DisplayType::State => DisplayType::State as u8,
            DisplayType::Details => DisplayType::Details as u8,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
