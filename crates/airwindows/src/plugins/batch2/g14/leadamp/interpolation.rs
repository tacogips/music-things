//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! LeadAmp undersampling interpolation, upstream revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.

pub(super) fn update(reference: &mut [f64; 10], sample: f64, cycle_end: usize) {
    match cycle_end {
        4 => {
            reference[0] = reference[4];
            reference[2] = (reference[0] + sample) / 2.0;
            reference[1] = (reference[0] + reference[2]) / 2.0;
            reference[3] = (reference[2] + sample) / 2.0;
            reference[4] = sample;
        }
        3 => {
            reference[0] = reference[3];
            reference[2] = (reference[0] + reference[0] + sample) / 3.0;
            reference[1] = (reference[0] + sample + sample) / 3.0;
            reference[3] = sample;
        }
        2 => {
            reference[0] = reference[2];
            reference[1] = (reference[0] + sample) / 2.0;
            reference[2] = sample;
        }
        _ => reference[0] = sample,
    }
}

pub(super) fn average(reference: &mut [f64; 10], mut sample: f64, cycle_end: usize) -> f64 {
    for stage in (2..=cycle_end).rev() {
        let index = stage + 3;
        reference[8] = sample;
        sample = (sample + reference[index]) * 0.5;
        reference[index] = reference[8];
    }
    sample
}
