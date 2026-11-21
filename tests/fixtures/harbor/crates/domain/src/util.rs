//! Small helpers.

pub fn clamp(value: u32, limit: u32) -> u32 {
    value.min(limit)
}
