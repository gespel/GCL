use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct SineSynth {
    pub freq: f32
}

impl SineSynth {
    pub fn play(&self) {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("Kein Standard-Ausgabegerät gefunden.");

        // Konfigurieren Sie das Stream-Format (Sinuston)
        let format = cpal::StreamConfig {
            channels: 2,  // Anzahl der Audiokanäle (2 für Stereo)
            buffer_size: cpal::BufferSize::Fixed(2048), // Puffergröße
            sample_rate: cpal::SampleRate(48000), // Beispiel-Sample-Rate (44100 Hz)
        };

        // Erstellen Sie einen Sinuston-Generator
        let sample_rate = format.sample_rate.0 as f32;
        let mut phase = 0.0;
        let frequency = self.freq; // Hier können Sie die Frequenz anpassen (440 Hz für A440).

        // Erstellen Sie den Stream und geben Sie den Sinuston aus
        let stream = device.build_output_stream(&format, move |data: &mut [f32], _| {
            for frame in data.chunks_mut(format.channels as usize) {
                let sample = f32::sin(2.0 * std::f32::consts::PI * frequency * phase / sample_rate);
                phase += 1.0 / sample_rate;
                frame[0] = sample;
                frame[1] = sample; // Für Stereo-Audio
            }
        }, |err| eprintln!("Fehler im Stream: {:?}", err), None).expect("Fehler beim Erstellen des Streams");

        // Starten des Streams
        stream.play().expect("Fehler beim Starten des Streams");

        println!("Drücken Sie Enter, um die Ausgabe zu beenden...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Fehler beim Lesen der Eingabe");

        // Stoppen Sie den Stream und beenden Sie das Programm
        stream.pause().expect("Fehler beim Pausieren des Streams");
        println!("Programm beendet.");
    }
}