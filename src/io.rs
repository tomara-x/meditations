use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, SizedSample,
};
use crossbeam_channel::{bounded, Receiver, Sender};
use fundsp::hacker32::*;

pub fn play(unit: Box<dyn AudioUnit>) {
    let host = cpal::default_host();
    if let Some(device) = host.default_output_device() {
        if let Ok(default_config) = device.default_output_config() {
            let mut config = default_config.config();
            config.channels = 2;
            match default_config.sample_format() {
                cpal::SampleFormat::F32 => run::<f32>(&device, &config, unit),
                cpal::SampleFormat::I16 => run::<i16>(&device, &config, unit),
                cpal::SampleFormat::U16 => run::<u16>(&device, &config, unit),
                format => eprintln!("unsupported sample format: {}", format),
            };
        }
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, unit: Box<dyn AudioUnit>)
where
    T: SizedSample + FromSample<f32>,
{
    let mut unit = BlockRateAdapter::new(unit);
    let mut next_value = move || {
        let (l, r) = unit.get_stereo();
        (
            if l.is_normal() { l.clamp(-1., 1.) } else { 0. },
            if r.is_normal() { r.clamp(-1., 1.) } else { 0. },
        )
    };
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| write_data(data, &mut next_value),
        err_fn,
        None,
    );
    if let Ok(stream) = stream {
        if let Ok(()) = stream.play() {
            std::mem::forget(stream)
        }
    }
}

fn write_data<T>(output: &mut [T], next_sample: &mut dyn FnMut() -> (f32, f32))
where
    T: SizedSample + FromSample<f32>,
{
    for frame in output.chunks_mut(2) {
        let sample = next_sample();
        frame[0] = T::from_sample(sample.0);
        frame[1] = T::from_sample(sample.1);
    }
}

/// mic input node
/// - output 0: left
/// - output 1: right
#[derive(Clone)]
pub struct InputNode {
    lr: Receiver<f32>,
    rr: Receiver<f32>,
}
impl InputNode {
    pub fn new(lr: Receiver<f32>, rr: Receiver<f32>) -> Self {
        InputNode { lr, rr }
    }
}
impl AudioNode for InputNode {
    const ID: u64 = 1117;
    type Inputs = U0;
    type Outputs = U2;

    #[inline]
    fn tick(&mut self, _input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let l = self.lr.try_recv().unwrap_or(0.);
        let r = self.rr.try_recv().unwrap_or(0.);
        [l, r].into()
    }
}

pub fn input() -> An<InputNode> {
    let (ls, lr) = bounded(4096);
    let (rs, rr) = bounded(4096);
    let host = cpal::default_host();
    if let Some(device) = host.default_input_device() {
        if let Ok(config) = device.default_input_config() {
            match config.sample_format() {
                cpal::SampleFormat::F32 => run_in::<f32>(&device, &config.into(), ls, rs),
                cpal::SampleFormat::I16 => run_in::<i16>(&device, &config.into(), ls, rs),
                cpal::SampleFormat::U16 => run_in::<u16>(&device, &config.into(), ls, rs),
                format => eprintln!("unsupported sample format: {}", format),
            };
        }
    }
    An(InputNode::new(lr, rr))
}

fn run_in<T>(device: &cpal::Device, config: &cpal::StreamConfig, ls: Sender<f32>, rs: Sender<f32>)
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let channels = config.channels as usize;
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
            read_data(data, channels, ls.clone(), rs.clone())
        },
        err_fn,
        None,
    );
    if let Ok(stream) = stream {
        if let Ok(()) = stream.play() {
            std::mem::forget(stream);
        }
    }
}

fn read_data<T>(input: &[T], channels: usize, ls: Sender<f32>, rs: Sender<f32>)
where
    T: SizedSample,
    f32: FromSample<T>,
{
    for frame in input.chunks(channels) {
        for (channel, sample) in frame.iter().enumerate() {
            if channel & 1 == 0 {
                let _ = ls.try_send(sample.to_sample::<f32>());
            } else {
                let _ = rs.try_send(sample.to_sample::<f32>());
            }
        }
    }
}
