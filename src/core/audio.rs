use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::time::Duration;

pub struct AudioBeep {
    sink: Sink,
}

impl AudioBeep {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = SineWave::new(700.0)
            .take_duration(Duration::from_secs_f32(2.5))
            .amplify(0.30);

        sink.append(source);
        sink.pause();

        AudioBeep { sink }
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }
}
