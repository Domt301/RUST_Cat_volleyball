use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};

use std::iter::Cycle;
use std::vec::IntoIter;

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle{
    println!("{}", file);
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialise_audio(world: &mut World) {
    use crate::catvolleyball::{AUDIO_BOUNCE, AUDIO_MUSIC, AUDIO_SCORE};
    println!("init music");

    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25);

        println!("Loading music");
        let music = AUDIO_MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };
        
        let sound = Sounds {
            bounce_sfx: load_audio_track(&loader, &world, AUDIO_BOUNCE),
            score_sfx: load_audio_track(&loader, &world, AUDIO_SCORE),
        };

        (sound, music)
    };





    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_bounce(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}