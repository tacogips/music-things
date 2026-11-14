//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Logical4, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::too_many_arguments
)]

mod first;
mod second;
mod third;
pub(super) struct Channel {
    delay: [f64; 1000],
    sag: f64,
    controls: [[f64; 2]; 2],
    targets: [f64; 2],
    averages: [f64; 2],
}
impl Channel {
    fn new() -> Self {
        Self {
            delay: [0.0; 1000],
            sag: 0.0,
            controls: [[1.0; 2]; 2],
            targets: [1.0; 2],
            averages: [0.0; 2],
        }
    }
}
pub(super) struct Stage {
    left: Channel,
    right: Channel,
}
impl Stage {
    pub(super) fn new() -> Self {
        Self {
            left: Channel::new(),
            right: Channel::new(),
        }
    }
}
