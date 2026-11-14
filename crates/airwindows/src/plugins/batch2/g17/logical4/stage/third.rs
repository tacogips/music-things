//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Logical4, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::manual_clamp
)]
use super::Stage;
impl Stage {
    pub(in super::super) fn third(
        &mut self,
        l: &mut f64,
        r: &mut f64,
        count: usize,
        offset: usize,
        divisor: f64,
        remainder: f64,
        flip: bool,
        gain: f64,
    ) {
        let old = 0.618033988749894848204586_f64;
        let new = 1.0 - old;
        // Power Sag left.
        let controls = self.left.controls[0][0]
            + self.left.controls[1][0]
            + self.left.controls[0][1]
            + self.left.controls[1][1];
        let value = (*l).abs() * (0.0445556 - (controls * 0.003300223685324102874217));
        self.left.delay[count] = value;
        self.left.delay[count + 499] = value;
        self.left.sag += self.left.delay[count] / offset as f64;
        self.left.sag -= self.left.delay[count + offset] / offset as f64;
        self.left.sag -= 0.000001;
        let mut clamp = 1.0;
        if self.left.sag < 0.0 {
            self.left.sag = 0.0;
        }
        if self.left.sag > 1.0 {
            clamp -= self.left.sag - 1.0;
            self.left.sag = 1.0;
        }
        if clamp < 0.5 {
            clamp = 0.5;
        }
        let thickness = ((1.0 - self.left.sag) * 2.0) - 1.0;
        let out = thickness.abs();
        let mut rect = (*l).abs();
        if rect > 1.57079633 {
            rect = 1.57079633;
        }
        if thickness > 0.0 {
            rect = rect.sin();
        } else {
            rect = 1.0 - rect.cos();
        }
        if *l > 0.0 {
            *l = *l * (1.0 - out) + rect * out;
        } else {
            *l = *l * (1.0 - out) - rect * out;
        }
        if clamp != 1.0 {
            *l *= clamp;
        }
        // Power Sag right.
        let controls = self.right.controls[0][0]
            + self.right.controls[1][0]
            + self.right.controls[0][1]
            + self.right.controls[1][1];
        let value = (*r).abs() * (0.0445556 - (controls * 0.003300223685324102874217));
        self.right.delay[count] = value;
        self.right.delay[count + 499] = value;
        self.right.sag += self.right.delay[count] / offset as f64;
        self.right.sag -= self.right.delay[count + offset] / offset as f64;
        self.right.sag -= 0.000001;
        let mut clamp = 1.0;
        if self.right.sag < 0.0 {
            self.right.sag = 0.0;
        }
        if self.right.sag > 1.0 {
            clamp -= self.right.sag - 1.0;
            self.right.sag = 1.0;
        }
        if clamp < 0.5 {
            clamp = 0.5;
        }
        let thickness = ((1.0 - self.right.sag) * 2.0) - 1.0;
        let out = thickness.abs();
        let mut rect = (*r).abs();
        if rect > 1.57079633 {
            rect = 1.57079633;
        }
        if thickness > 0.0 {
            rect = rect.sin();
        } else {
            rect = 1.0 - rect.cos();
        }
        if *r > 0.0 {
            *r = *r * (1.0 - out) + rect * out;
        } else {
            *r = *r * (1.0 - out) - rect * out;
        }
        if clamp != 1.0 {
            *r *= clamp;
        }
        // Compressor left.
        let _ = gain;
        let mut input_pos = (*l * old) + (self.left.averages[0] * new) + 1.0;
        self.left.averages[0] = *l;
        if input_pos < 0.001 {
            input_pos = 0.001;
        }
        let mut output_pos = input_pos / 2.0;
        if output_pos > 1.0 {
            output_pos = 1.0;
        }
        input_pos *= input_pos;
        let mut dynamic_pos = remainder * (input_pos + 1.0);
        if dynamic_pos > 1.0 {
            dynamic_pos = 1.0;
        }
        let dynamic_divisor_pos = 1.0 - dynamic_pos;
        self.left.targets[0] *= dynamic_divisor_pos;
        self.left.targets[0] += input_pos * dynamic_pos;
        let calc_pos = (1.0 / self.left.targets[0]).powf(2.0);
        let mut input_neg = (-*l * old) + (self.left.averages[1] * new) + 1.0;
        self.left.averages[1] = -*l;
        if input_neg < 0.001 {
            input_neg = 0.001;
        }
        let mut output_neg = input_neg / 2.0;
        if output_neg > 1.0 {
            output_neg = 1.0;
        }
        input_neg *= input_neg;
        let mut dynamic_neg = remainder * (input_neg + 1.0);
        if dynamic_neg > 1.0 {
            dynamic_neg = 1.0;
        }
        let dynamic_divisor_neg = 1.0 - dynamic_neg;
        self.left.targets[1] *= dynamic_divisor_neg;
        self.left.targets[1] += input_neg * dynamic_neg;
        let calc_neg = (1.0 / self.left.targets[1]).powf(2.0);
        if *l > 0.0 {
            if flip {
                self.left.controls[0][0] *= divisor;
                self.left.controls[0][0] += calc_pos * remainder;
                if self.right.controls[0][0] > self.left.controls[0][0] {
                    self.right.controls[0][0] =
                        (self.right.controls[0][0] + self.left.controls[0][0]) * 0.5;
                }
            } else {
                self.left.controls[1][0] *= divisor;
                self.left.controls[1][0] += calc_pos * remainder;
                if self.right.controls[1][0] > self.left.controls[1][0] {
                    self.right.controls[1][0] =
                        (self.right.controls[1][0] + self.left.controls[1][0]) * 0.5;
                }
            }
        } else {
            if flip {
                self.left.controls[0][1] *= divisor;
                self.left.controls[0][1] += calc_neg * remainder;
                if self.right.controls[0][1] > self.left.controls[0][1] {
                    self.right.controls[0][1] =
                        (self.right.controls[0][1] + self.left.controls[0][1]) * 0.5;
                }
            } else {
                self.left.controls[1][1] *= divisor;
                self.left.controls[1][1] += calc_neg * remainder;
                if self.right.controls[1][1] > self.left.controls[1][1] {
                    self.right.controls[1][1] =
                        (self.right.controls[1][1] + self.left.controls[1][1]) * 0.5;
                }
            }
        }
        let total = if flip {
            self.left.controls[0][0] * output_pos + self.left.controls[0][1] * output_neg
        } else {
            self.left.controls[1][0] * output_pos + self.left.controls[1][1] * output_neg
        };
        if total != 1.0 {
            *l *= total;
        }
        if *l > 36.0 {
            *l = 36.0;
        }
        if *l < -36.0 {
            *l = -36.0;
        }
        // Compressor right.
        let mut input_pos = (*r * old) + (self.right.averages[0] * new) + 1.0;
        self.right.averages[0] = *r;
        if input_pos < 0.001 {
            input_pos = 0.001;
        }
        let mut output_pos = input_pos / 2.0;
        if output_pos > 1.0 {
            output_pos = 1.0;
        }
        input_pos *= input_pos;
        let mut dynamic_pos = remainder * (input_pos + 1.0);
        if dynamic_pos > 1.0 {
            dynamic_pos = 1.0;
        }
        let dynamic_divisor_pos = 1.0 - dynamic_pos;
        self.left.targets[0] *= dynamic_divisor_pos;
        self.left.targets[0] += input_pos * dynamic_pos;
        let calc_pos = (1.0 / self.right.targets[0]).powf(2.0);
        let mut input_neg = (-*r * old) + (self.right.averages[1] * new) + 1.0;
        self.right.averages[1] = -*r;
        if input_neg < 0.001 {
            input_neg = 0.001;
        }
        let mut output_neg = input_neg / 2.0;
        if output_neg > 1.0 {
            output_neg = 1.0;
        }
        input_neg *= input_neg;
        let mut dynamic_neg = remainder * (input_neg + 1.0);
        if dynamic_neg > 1.0 {
            dynamic_neg = 1.0;
        }
        let dynamic_divisor_neg = 1.0 - dynamic_neg;
        self.right.targets[1] *= dynamic_divisor_neg;
        self.right.targets[1] += input_neg * dynamic_neg;
        let calc_neg = (1.0 / self.right.targets[1]).powf(2.0);
        if *r > 0.0 {
            if flip {
                self.right.controls[0][0] *= divisor;
                self.right.controls[0][0] += calc_pos * remainder;
                if self.left.controls[0][0] > self.right.controls[0][0] {
                    self.left.controls[0][0] =
                        (self.right.controls[0][0] + self.left.controls[0][0]) * 0.5;
                }
            } else {
                self.right.controls[1][0] *= divisor;
                self.right.controls[1][0] += calc_pos * remainder;
                if self.left.controls[1][0] > self.right.controls[1][0] {
                    self.left.controls[1][0] =
                        (self.right.controls[1][0] + self.left.controls[1][0]) * 0.5;
                }
            }
        } else {
            if flip {
                self.right.controls[0][1] *= divisor;
                self.right.controls[0][1] += calc_neg * remainder;
                if self.left.controls[0][1] > self.right.controls[0][1] {
                    self.left.controls[0][1] =
                        (self.right.controls[0][1] + self.left.controls[0][1]) * 0.5;
                }
            } else {
                self.right.controls[1][1] *= divisor;
                self.right.controls[1][1] += calc_neg * remainder;
                if self.left.controls[1][1] > self.right.controls[1][1] {
                    self.left.controls[1][1] =
                        (self.right.controls[1][1] + self.left.controls[1][1]) * 0.5;
                }
            }
        }
        let total = if flip {
            self.right.controls[0][0] * output_pos + self.right.controls[0][1] * output_neg
        } else {
            self.right.controls[1][0] * output_pos + self.right.controls[1][1] * output_neg
        };
        if total != 1.0 {
            *r *= total;
        }
        if *r > 36.0 {
            *r = 36.0;
        }
        if *r < -36.0 {
            *r = -36.0;
        }
    }
}
