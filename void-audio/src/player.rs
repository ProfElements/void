
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum AudioFormat {
    Mono8Bit = 0,
    Mono8BitUnsigned = 4,
    Mono16Bit = 1,
    Mono16BitLe = 5,
    Stereo8Bit = 2,
    Stereo8BitUnsigend = 6,
    Stereo16Bit = 3,
    Stereo16BitLe = 7,
}

#[derive(Copy, Clone)]
pub enum AudioFrequency {
    FourtyEightHz,
    ThirtyTwoHz,
    FourtyFourOneHz,
    Custom(u32),
}

impl From<u32> for AudioFrequency {
    fn from(val: u32) -> Self {
       match val {
            48000 => AudioFrequency::FourtyEightHz,
            44100 => AudioFrequency::FourtyFourOneHz,
            32000 => AudioFrequency::ThirtyTwoHz,
            _ => AudioFrequency::Custom(val)
       } 
    }         
}

pub struct AudioOptions {
    pub format: AudioFormat,
    pub frequency: AudioFrequency,
    pub volume: (f32, f32), 
}

impl AudioOptions {
    pub fn new() -> Self {
        Self {
            format: AudioFormat::Stereo16Bit,
            frequency: AudioFrequency::FourtyEightHz,
            volume: (1.0, 1.0)
        }
    }

    #[must_use]
    pub fn format(mut self, audio_format: AudioFormat) -> Self {
        self.format = audio_format;
        self
    }

    #[must_use]
    pub fn frequency(mut self, frequency: AudioFrequency) -> Self {
        self.frequency = frequency;
        self
    }

    #[must_use]
    pub fn volume(mut self, volume: (f32, f32)) -> Self {
        self.volume = volume;
        self
    }

    #[must_use]
    pub fn volume_left(mut self, volume: f32) -> Self {
        self.volume.0 = volume;
        self
    }
    
    #[must_use]
    pub fn volume_right(mut self, volume: f32) -> Self {
        self.volume.1 = volume;
        self
    }
}

pub trait AudioPlayer {
    type Error;
    fn play_pcm_buffer(&self, options: AudioOptions, delay: u32, buffer: &mut [u8]) -> Result<(), Self::Error>;
}
