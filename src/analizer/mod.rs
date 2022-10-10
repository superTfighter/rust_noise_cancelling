
// Perform a forward FFT of size 1234
use rustfft::{num_complex::Complex, FftPlanner};


#[derive(Debug)]
pub struct Analizer{

    s: String
}



impl Analizer{

    pub fn new() -> Analizer {
        Analizer {
            s: "Hello!".to_string()
        }
    }

    pub fn message(&self) -> String{

        format!("{}",self.s)
    }

    pub fn samples_fft_to_spectrum(
        samples: &[f32],
        sampling_rate: u32,
        frequency_limit: FrequencyLimit,
    ) -> Result<FrequencySpectrum, SpectrumAnalyzerError> {
        // everything below two samples is unreasonable
        if samples.len() < 2 {
            return Err(SpectrumAnalyzerError::TooFewSamples);
        }
        // do several checks on input data
        if samples.iter().any(|x| x.is_nan()) {
            return Err(SpectrumAnalyzerError::NaNValuesNotSupported);
        }
        if samples.iter().any(|x| x.is_infinite()) {
            return Err(SpectrumAnalyzerError::InfinityValuesNotSupported);
        }
        if !is_power_of_two(samples.len()) {
            return Err(SpectrumAnalyzerError::SamplesLengthNotAPowerOfTwo);
        }
        let max_detectable_frequency = sampling_rate as f32 / 2.0;
        // verify frequency limit: unwrap error or else ok
        frequency_limit
            .verify(max_detectable_frequency)
            .map_err(SpectrumAnalyzerError::InvalidFrequencyLimit)?;
    
        // With FFT we transform an array of time-domain waveform samples
        // into an array of frequency-domain spectrum samples
        // https://www.youtube.com/watch?v=z7X6jgFnB6Y
    
        // FFT result has same length as input
        // (but when we interpret the result, we don't need all indices)
    
        // applies the f32 samples onto the FFT algorithm implementation
        // chosen at compile time (via Cargo feature).
        // If a complex FFT implementation was chosen, this will internally
        // transform all data to Complex numbers.
        let buffer = FftImpl::fft_apply(samples);
    
        // This function:
        // 1) calculates the corresponding frequency of each index in the FFT result
        // 2) filters out unwanted frequencies
        // 3) calculates the magnitude (absolute value) at each frequency index for each complex value
        // 4) optionally scales the magnitudes
        // 5) collects everything into the struct "FrequencySpectrum"
        fft_result_to_spectrum(
            samples.len(),
            &buffer,
            sampling_rate,
            frequency_limit,
            scaling_fn,
        )
    }
}