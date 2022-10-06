extern crate portaudio;

use portaudio as pa;

use rustfft::{num_complex::Complex, FftPlanner};

use poloto::prelude::*;
use std::fs::File;
use std::io::prelude::*;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;
const INTERLEAVED: bool = true;
const FRAMES: u32 = 256;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), pa::Error> {
    let pa = pa::PortAudio::new()?;

    let def_input = pa.default_input_device()?;
    let input_info = pa.device_info(def_input)?;

    let latency = input_info.default_low_input_latency;
    let input_params = pa::StreamParameters::<f32>::new(def_input, CHANNELS, INTERLEAVED, latency);

    let def_output = pa.default_output_device()?;
    let output_info = pa.device_info(def_output)?;

    let latency = output_info.default_low_output_latency;
    let output_params = pa::StreamParameters::new(def_output, CHANNELS, INTERLEAVED, latency);

    pa.is_duplex_format_supported(input_params, output_params, SAMPLE_RATE)?;

    let settings = pa::DuplexStreamSettings::new(input_params, output_params, SAMPLE_RATE, FRAMES);

    // let (sender, receiver) = ::std::sync::mpsc::channel();
    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let callback = move |pa::DuplexStreamCallbackArgs {
                             in_buffer,
                             out_buffer,
                             frames,
                             time,
                             ..
                         }| {
        let mut buffer = vec![Complex { re: 0.0, im: 0.0 }; 256];

        let mut counter = 0;

        while counter < 512 {
            if counter == 0 {
                buffer[counter] = Complex {
                    re: in_buffer[counter],
                    im: 0.0,
                };
            } else if counter % 2 == 0 {
                buffer[counter / 2] = Complex {
                    re: in_buffer[counter],
                    im: 0.0,
                };
            }

            counter += 1;
        }

        calculate_audio(buffer.as_mut_slice());

        // Pass the input straight to the output - BEWARE OF FEEDBACK!
        for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
            *output_sample = *input_sample;
        }

        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    while let true = stream.is_active()? {
        // Do some stuff!

        //println!("ASD");
    }

    stream.stop()?;
    stream.close()?;

    Ok(())
}

fn calculate_audio(mut buffer: &mut [Complex<f32>]) {
    let mut planner = FftPlanner::<f32>::new();

    let fft = planner.plan_fft_forward(256);

    fft.process(&mut buffer);

    write_to_file(plot_fft(buffer));


}

fn write_to_file(data: String) {
    let mut file = File::create("result.svg").expect("ASD");

    file.write_all(data.as_bytes()).expect("ASD");
}

fn plot_fft(buffer: &mut [Complex<f32>]) -> String {

    


    "ASD".to_string()
}
