extern crate portaudio;

use portaudio as pa;
use portaudio::stream::Buffer;

// Perform a forward FFT of size 1234
use rustfft::{num_complex::Complex, FftPlanner};

use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit, FrequencySpectrum};

use poloto::prelude::*;

use std::fs::File;
use std::io::prelude::*;

use rand::prelude::*;

mod analizer;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;
const INTERLEAVED: bool = true;
const FRAMES: u32 = 256;

fn main() {

    /*let mut rng = rand::thread_rng();
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(256);
    let ifft = planner.plan_fft_inverse(256);

    let mut buffer:Vec<Complex<f32>> = vec![Complex{ re: 0.0, im: 0.0 }; 256];

    println!("Input---------------------------------------------------------------------------------------------- ");
    for i in 0..256 {

        let y: f32 = rng.gen_range(1.0..101.0);
        buffer[i] = Complex{re: y, im: 0.0};

        println!("{}" , buffer[i].re);
    }

    fft.process(&mut buffer);
    println!("After---------------------------------------------------------------------------------------------- ");
    for i in buffer.iter() {
        println!("{} ", i.re);
    }


    ifft.process(&mut buffer);
    println!("Ifft---------------------------------------------------------------------------------------------- ");
    for i in buffer.iter() {
        println!("{} ", (i.re / 256.0));
    }

    panic!("Test");*/
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

    let callback = move |pa::DuplexStreamCallbackArgs {
                             in_buffer,
                             out_buffer,
                             ..
                         }| {
        let mut buffer = vec![0.0; 256];

        let mut counter = 0;

        while counter < 512 {
            if counter == 0 {
                buffer[counter] = in_buffer[counter];
            } else if counter % 2 == 0 {
                buffer[counter / 2] = in_buffer[counter];
            }

            counter += 1;
        }

        calculate_audio(buffer);

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

fn calculate_audio(mut buffer: Vec<f32>) {  
  
    
    let mut count = 0;
    
    for val in &buffer
    {
        count += 1;
    }

    println!("{}" , count);

    
    let spectrum_hann_window = samples_fft_to_spectrum(
        // (windowed) samples
        &buffer,
        // sampling rate
        44100,
        // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
        FrequencyLimit::All,
        // optional scale
        None,
    )
    .unwrap();


    let mut count = 0;
    
    for (freq,freq_val) in spectrum_hann_window.data().iter()
    {
        count += 1;
    }

    println!("{}" , count);

    let analizer = analizer::Analizer::new();

    println!("{}", analizer.message());

    panic!("Die");

   

    //write_to_file(plot_fft(spectrum_hann_window));
}

fn write_to_file(data: String) {


    let mut rng = rand::thread_rng();

    let randomNumber =  rng.gen_range(1..999999);

    let filename = format!("results/result{}.svg",randomNumber);

    let mut file = File::create(filename).expect("ASD");

    file.write_all(data.as_bytes()).expect("ASD");
}

fn plot_fft(buffer: FrequencySpectrum) -> String {
    let mut data: [(f64, f64); 256] = [(0.0, 0.0); 256];
    let mut index: i32 = 0;

    for (fr, fr_val) in buffer.data().iter() {
        data[index as usize] = (fr.val() as f64, fr_val.val() as f64);

        index += 1;
    }

    let plots = poloto::plots!(
        poloto::build::plot("").line().cloned(data.iter()),
        poloto::build::markers([], [])
    );

    let a = poloto::data(plots)
        .build_and_label(("FFT", "X", "Y"))
        .append_to(poloto::header().light_theme())
        .render_string()
        .expect("Cannot convert to string!");

    // let a = "ASD".to_string();
    a
}
