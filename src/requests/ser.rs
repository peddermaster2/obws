use chrono::Duration;
use rgb::RGBA8;
use serde::ser::{Error, Serializer};

pub fn duration_millis_opt<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(duration) => serializer.serialize_some(&duration.num_milliseconds()),
        None => serializer.serialize_none(),
    }
}

pub fn duration_millis<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(value.num_milliseconds())
}

pub fn duration_nanos<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value.num_nanoseconds() {
        Some(nanos) => serializer.serialize_i64(nanos),
        None => Err(Error::custom(
            "duration is too big to be serialized as nanoseconds",
        )),
    }
}

pub fn bitflags_u8_opt<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<u8> + Copy,
{
    match value {
        Some(flags) => serializer.serialize_some(&(*flags).into()),
        None => serializer.serialize_none(),
    }
}

pub fn rgba8_inverse_opt<S>(value: &Option<RGBA8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(rgba) => {
            let abgr = (rgba.a as u32) << 24
                | (rgba.b as u32) << 16
                | (rgba.g as u32) << 8
                | (rgba.r as u32);
            serializer.serialize_some(&abgr)
        }
        None => serializer.serialize_none(),
    }
}
