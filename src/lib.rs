pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(feature = "asr")]
pub use airs_asr as asr;

#[cfg(feature = "audio")]
pub use airs_audio as audio;

#[cfg(feature = "gui")]
pub use airs_gui as gui;

#[cfg(feature = "hash")]
pub use airs_hash as hash;

#[cfg(feature = "image")]
pub use airs_image as image;

#[cfg(feature = "tts")]
pub use airs_tts as tts;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches_package_version() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    }
}
