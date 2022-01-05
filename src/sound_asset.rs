/*
pub struct SoundAsset<'a>  {
    buffer: Option<sfml::SfBox<sfml::audio::SoundBuffer>>,
    sound: Option<sfml::audio::Sound<'a>>,
}

impl<'a> SoundAsset<'a> {
    pub fn new(filename: &str) -> SoundAsset<'a> {
        let mut asset = SoundAsset{
            buffer: sfml::audio::SoundBuffer::from_file(filename),
            sound: None,
        };
        asset.sound = Some(sfml::audio::Sound::with_buffer(&asset.buffer.unwrap()));
        asset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sound() {
        let sound = SoundAsset::new("assets/examples_resources_ball.wav");
        assert_eq!(1, 2);
    }
}*/