use std::fs::File;
use std::path::Path;

use audrey::read::Reader;
use dasp_interpolate::linear::Linear;
use dasp_signal::{from_iter, interpolate::Converter, Signal};
use deepspeech::Model;
use std::convert::TryInto;
use std::io::Write;
use anyhow::Error;

// The model has been trained on this specific
// sample rate.
const SAMPLE_RATE: u32 = 16_000;

// pub fn new_model() -> Result<Model, anyhow::Error> {
//
// }

pub trait New {
    fn new () -> Result<Self, anyhow::Error>;
}

impl New for Model {
    fn new () -> Result<Self, anyhow::Error> {
        let model_dir_str = std::env::var("MODEL_DIR_PATH").expect("Please specify model dir");
        let dir_path = Path::new(&model_dir_str);
        let mut graph_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
        let mut scorer_name: Option<Box<Path>> = None;
        for file in dir_path
            .read_dir()
            .expect("Specified model dir is not a dir")
        {
            if let Ok(f) = file {
                let file_path = f.path();
                if file_path.is_file() {
                    if let Some(ext) = file_path.extension() {
                        if ext == "pb" || ext == "pbmm" || ext == "tflite" {
                            graph_name = file_path.into_boxed_path();
                        } else if ext == "scorer" {
                            scorer_name = Some(file_path.into_boxed_path());
                        }
                    }
                }
            }
        }
        let mut m = Model::load_from_files(&graph_name).unwrap();
        // enable external scorer if found in the model folder
        if let Some(scorer) = scorer_name {
            println!("Using external scorer `{}`", scorer.to_str().unwrap());
            m.enable_external_scorer(&scorer).unwrap();
        };
        Ok(m)
    }
}

pub trait Transcript {
    fn transcript(&self, snippet: Vec<u8>) -> Result<String, anyhow::Error>;
}

impl Transcript for Model {
    fn transcript(&mut self, snippet: Vec<u8>) -> Result<String, Error> {
        let snippet = read(write(snippet)?)?;
        let result = self.speech_to_text(&snippet)?;
        Ok(result)
    }
}
//not so smart for now

fn write(snippet: Vec<u8>) -> Result<File, anyhow::Error> {
    let mut path = std::env::var("WAV_DIR_PATH").unwrap();
    let mut file = File::create(format!("{}/temp.wav", path.as_str())).unwrap();
    File::write_all(&mut file, snippet.as_slice());
    Ok(file)
}

fn read(file: File) -> Result<Vec<i16>, anyhow::Error> {
    let mut reader = Reader::new(file).unwrap();
    let desc = reader.description();

    assert_eq!(
        1,
        desc.channel_count(),
        "The channel count is required to be one, at least for now"
    );

    let audio_buf: Vec<_> = if desc.sample_rate() == SAMPLE_RATE {
        reader.samples().map(|s| s.unwrap()).collect()
    } else {
        // We need to interpolate to the target sample rate
        let interpolator = Linear::new([0i16], [0]);
        let conv = Converter::from_hz_to_hz(
            from_iter(reader.samples::<i16>().map(|s| [s.unwrap()])),
            interpolator,
            desc.sample_rate() as f64,
            SAMPLE_RATE as f64,
        );
        conv.until_exhausted().map(|v| v[0]).collect()
    };
    Ok(audio_buf)
}

#[cfg(test)]
mod test {}
