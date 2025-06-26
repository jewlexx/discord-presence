/// [`ActivityType`] enum
///
/// Lists all activity types currently supported by Discord.
///
/// This may change in future if Discord adds support for more types,
/// or removes support for some.
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActivityType {
    /// Playing a game
    Playing = 0,
    /// Listening to...
    Listening = 2,
    /// Watching...
    Watching = 3,
    /// Competing in...
    Competing = 5,
}

impl<'de> serde::Deserialize<'de> for ActivityType {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const PLAYING: u8 = ActivityType::Playing as u8;
        const LISTENING: u8 = ActivityType::Listening as u8;
        const WATCHING: u8 = ActivityType::Watching as u8;
        const COMPETING: u8 = ActivityType::Competing as u8;

        match u8::deserialize(deserializer)? {
            PLAYING => Result::Ok(ActivityType::Playing),
            LISTENING => Result::Ok(ActivityType::Listening),
            WATCHING => Result::Ok(ActivityType::Watching),
            COMPETING => Result::Ok(ActivityType::Competing),
            other => Result::Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(u64::from(other)),
                &(&format!("one of: {PLAYING}, {LISTENING}, {WATCHING}, {COMPETING}") as &str),
            )),
        }
    }
}

impl serde::Serialize for ActivityType {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match *self {
            ActivityType::Playing => ActivityType::Playing as u8,
            ActivityType::Listening => ActivityType::Listening as u8,
            ActivityType::Watching => ActivityType::Watching as u8,
            ActivityType::Competing => ActivityType::Competing as u8,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
