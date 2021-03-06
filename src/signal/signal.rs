use dasp::Frame;
use dasp_signal::Signal;

use dasp_sample::Sample;

use super::frame::*;
use super::generator::SignalGenerator;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SterioSignal {
    frames: Vec<SterioFrame>,
}

impl SterioSignal {
    pub fn new() -> SterioSignal {
        SterioSignal { frames: Vec::new() }
    }

    pub fn from_generator<T: SignalGenerator>(gen: T, num_frames: usize) -> SterioSignal {
        SterioSignal {
            frames: {
                let mut n_frames: Vec<SterioFrame> = Vec::new();
                for i in 0..num_frames {
                    n_frames.push(gen.generate_frame(i));
                }
                n_frames
            },
        }
    }

    pub fn get_raw_frame(&self, idx: usize) -> Option<&SterioFrame> {
        self.frames.get(idx).clone()
    }

    pub fn get_num_frames(&self) -> usize {
        self.frames.len()
    }

    pub fn push_sterio_frame(&mut self, frame: SterioFrame) -> () {
        self.frames.push(frame);
    }
}

impl<T: dasp::Frame> From<&mut dyn Signal<Frame = T>> for SterioSignal
where
    f32: dasp::sample::FromSample<<T as dasp::Frame>::Sample>,
{
    fn from(from_signal: &mut dyn Signal<Frame = T>) -> Self {
        let mut sterio_signal = SterioSignal::new();
        loop {
            let frame: T = from_signal.next();
            match from_signal.is_exhausted() {
                true => break,
                false => {
                    let mut mono_frame: f32 = 0.0;
                    for sample in frame.channels() {
                        mono_frame += sample.to_sample::<f32>();
                    }
                    sterio_signal
                        .frames
                        .push(SterioFrame::from([mono_frame, mono_frame]));
                }
            };
        }
        sterio_signal
    }
}

impl Signal for SterioSignal {
    type Frame = SterioFrame;
    fn next(&mut self) -> Self::Frame {
        self.frames
            .clone()
            .into_iter()
            .next()
            .unwrap_or(Self::Frame::EQUILIBRIUM)
    }
}
