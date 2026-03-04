use serde::{Deserialize, Deserializer};

/// Deserializer for `Option<Option<T>>` fields in PATCH payloads.
///
/// Distinguishes between a field being absent (keep existing value) and
/// explicitly set to `null` (clear the value):
///
/// - Field absent from JSON  → outer `None`  → keep existing DB value
/// - Field present as `null` → `Some(None)`  → set DB column to NULL
/// - Field present as value  → `Some(Some(v))` → set DB column to value
///
/// Usage: `#[serde(default, deserialize_with = "crate::serde_helpers::double_option")]`
pub fn double_option<'de, T, D>(de: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(de).map(Some)
}
