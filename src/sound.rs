/*
pub struct Sound<'a>  {
    ball_soundbuffer: sfml::SfBox<sfml::audio::SoundBuffer>,
    ball_sound: std::cell::RefCell<sfml::audio::Sound<'a>>,
}

impl<'a> Sound<'a> {
    pub fn new() -> Sound<'a> {
        let ball_soundbuffer = sfml::audio::SoundBuffer::from_file("assets/examples_resources_ball.wav").unwrap();
        let ball_sound = sfml::audio::Sound::with_buffer(&ball_soundbuffer);
        Sound{
            ball_soundbuffer,
            ball_sound,
        }
    }
}
*/