use ogc_rs::{
    asnd::{Asnd, VoiceFormat, VoiceOptions},
    OgcError,
};
use void_audio::player::{AudioPlayer, AudioOptions, AudioFrequency, AudioFormat};

pub struct Audio {
    _asnd: Asnd,
}

impl Audio {
    pub fn new() -> Self {
        let asnd = Asnd::init();
        Asnd::pause(false);
        Self { _asnd: asnd }
    }
    pub fn play_buffer(&self, format: VoiceFormat, rate: u32, volume: (u8, u8), buffer: &mut [u8]) {
        Asnd::set_voice(
            VoiceOptions::new()
                .voice(Asnd::get_first_unused_voice().unwrap())
                .format(format)
                .pitch(rate)
                .volume_left(volume.0)
                .volume_right(volume.1),
            buffer,
        )
        .unwrap();
    }
}

impl AudioPlayer for Audio {
    type Error = OgcError;
    fn play_pcm_buffer(
        &self,
        opts: AudioOptions,
        _delay: u32,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        let voice_f: VoiceF = AudioF(opts.format).into();
        let freq = match opts.frequency {
            AudioFrequency::Custom(val) => val,
            AudioFrequency::ThirtyTwoHz => 32000,
            AudioFrequency::FourtyEightHz => 48000,
            AudioFrequency::FourtyFourOneHz => 44100,
        };

        self.play_buffer(
            voice_f.0,
            freq,
            ((opts.volume.0 * 255.0) as u8, (opts.volume.1 * 255.0) as u8),
            buffer,
        );
        Ok(())
    }
}

pub struct VoiceF(VoiceFormat);
pub struct AudioF(AudioFormat);

impl From<AudioF> for VoiceF {
    fn from(format: AudioF) -> Self {
        match format.0 {
            AudioFormat::Mono8Bit => VoiceF(VoiceFormat::Mono8Bit),
            AudioFormat::Mono16Bit => VoiceF(VoiceFormat::Mono16Bit),
            AudioFormat::Mono16BitLe => VoiceF(VoiceFormat::Mono16BitLE),
            AudioFormat::Stereo8Bit => VoiceF(VoiceFormat::Stereo8Bit),
            AudioFormat::Stereo16Bit => VoiceF(VoiceFormat::Stereo16Bit),
            AudioFormat::Stereo16BitLe => VoiceF(VoiceFormat::Stereo16BitLe),
            AudioFormat::Mono8BitUnsigned => VoiceF(VoiceFormat::Mono8BitU),
            AudioFormat::Stereo8BitUnsigend => VoiceF(VoiceFormat::Stereo8BitU),
        }
    }
}
