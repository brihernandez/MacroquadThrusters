use macroquad::audio::{self, PlaySoundParams, Sound};
use macroquad::rand::ChooseRandom;

pub struct GameSounds {
    ambience: Vec<Sound>,
    impact: Vec<Sound>,
    sci_fear: Vec<Sound>,
    jet: Sound,
    is_jet_playing: bool,
}

impl GameSounds {
    pub async fn new() -> Self {
        GameSounds {
            ambience: vec![
                audio::load_sound("labdrone1.wav").await.unwrap(),
                audio::load_sound("computalk1.wav").await.unwrap(),
            ],
            impact: vec![
                audio::load_sound("bustglass2.wav").await.unwrap(),
                audio::load_sound("bustglass3.wav").await.unwrap(),
                audio::load_sound("bustmetal1.wav").await.unwrap(),
                audio::load_sound("bustmetal2.wav").await.unwrap(),
                audio::load_sound("bustconcrete1.wav").await.unwrap(),
                audio::load_sound("bustconcrete2.wav").await.unwrap(),
                audio::load_sound("bustcrate1.wav").await.unwrap(),
                audio::load_sound("bustcrate2.wav").await.unwrap(),
                audio::load_sound("bustcrate3.wav").await.unwrap(),
            ],
            sci_fear: vec![
                audio::load_sound("sci_fear1.wav").await.unwrap(),
                audio::load_sound("sci_fear2.wav").await.unwrap(),
                audio::load_sound("sci_fear3.wav").await.unwrap(),
                audio::load_sound("sci_fear4.wav").await.unwrap(),
                audio::load_sound("sci_fear5.wav").await.unwrap(),
                audio::load_sound("sci_fear6.wav").await.unwrap(),
                audio::load_sound("sci_fear7.wav").await.unwrap(),
                audio::load_sound("sci_fear8.wav").await.unwrap(),
                audio::load_sound("sci_fear9.wav").await.unwrap(),
                audio::load_sound("sci_fear10.wav").await.unwrap(),
                audio::load_sound("sci_fear11.wav").await.unwrap(),
                audio::load_sound("sci_fear12.wav").await.unwrap(),
                audio::load_sound("sci_fear13.wav").await.unwrap(),
                audio::load_sound("sci_fear14.wav").await.unwrap(),
                audio::load_sound("sci_fear15.wav").await.unwrap(),
                audio::load_sound("sci_pain1.wav").await.unwrap(),
                audio::load_sound("sci_pain2.wav").await.unwrap(),
                audio::load_sound("sci_pain3.wav").await.unwrap(),
                audio::load_sound("sci_pain4.wav").await.unwrap(),
                audio::load_sound("sci_pain5.wav").await.unwrap(),
                audio::load_sound("sci_pain6.wav").await.unwrap(),
                audio::load_sound("sci_pain7.wav").await.unwrap(),
                audio::load_sound("sci_pain8.wav").await.unwrap(),
                audio::load_sound("sci_pain9.wav").await.unwrap(),
                audio::load_sound("sci_pain10.wav").await.unwrap(),
                audio::load_sound("stopattacking.wav").await.unwrap(),
            ],
            jet: audio::load_sound("steamjet1.wav").await.unwrap(),
            is_jet_playing: false,
        }
    }

    pub fn play_looping_ambience(&self) {
        for ambient_sound in self.ambience.iter() {
            audio::play_sound(
                &ambient_sound,
                PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
        }
    }

    pub fn play_impact(&self) {
        let sound = self.impact.choose().unwrap();
        audio::play_sound_once(&sound);
    }

    pub fn play_sciencist_fear(&self) {
        let sound = self.sci_fear.choose().unwrap();
        audio::play_sound_once(&sound);
    }

    pub fn play_jet(&mut self) {
        audio::play_sound(
            &self.jet,
            PlaySoundParams {
                looped: true,
                volume: 0.33,
            },
        );
        self.is_jet_playing = true;
    }

    pub fn stop_jet(&mut self) {
        audio::stop_sound(&self.jet);
        self.is_jet_playing = false;
    }

    pub fn get_is_jet_playing(&self) -> bool {
        return self.is_jet_playing;
    }
}
