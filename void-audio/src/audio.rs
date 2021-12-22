use crate::player::AudioPlayer;


/// TODO: Make Atleast MP3, WAV, OGG, and MOD files to implement this when I get no_std audio
/// stuff.
pub trait Audio {
    fn play<P>(&self, player: P) -> Result<(), P::Error> where P: AudioPlayer;
}
