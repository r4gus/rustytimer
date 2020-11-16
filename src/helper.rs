/// Extract the seconds part from at time span given in seconds.
pub fn seconds(t: u64) -> u64 {
    t % 60
}

/// Extract the minutes from a time span given in seconds.
pub fn minutes(t: u64) -> u64 {
    (t % 3600) / 60
}

/// Extract the hours from a time span given in seconds.
pub fn hours(t: u64) -> u64 {
    t / 3600
}