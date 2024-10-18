// lapis

let seq = Sequencer::new(false, 1);
let be = seq.backend();
let verb = multipass::<U2>() & reverb_stereo(60,10,0.4);
let g = be * 0.5 >> split::<U2>() >> verb;

let bpm = 115;
let beat = bpm / 4 / 60;
let mid_c = pow(2, -9/12) * 440;
let scale = [1,4,7,8,14];

g.play();

let kick = bassdrum(0.5, 1000, 20);

for i in 0..16 {
  let s = i*beat;
  let hz = semitone_ratio(i) * mid_c;
  if i % 2 == 0 {
    let e = seq.push_relative(
      s, s+beat, Fade::Smooth, 0.01, 0.01, sine_hz(hz)
    );
  } else {
    let e = seq.push_relative(
      s, s+beat/2, Fade::Smooth, 0.01, 0.01, sine_hz(hz*2)
    );
  }
}

seq.push_relative(0, beat, Fade::Smooth, 0.01, 0.01, kick.clone());

for i in 0..8 {
  let s = beat*i*4;
  let kick = bassdrum(0.9, 3000, 20) * 32;
  seq.push_relative(s, s+beat, Fade::Smooth, 0.01, 0.01, kick);
}

for i in 0..16 {
  let s = i*beat;
  let hz = semitone_ratio(scale[i % 5]) * mid_c;
  let e = seq.push_relative(
    s, s+beat/2, Fade::Smooth, 0.01, 0.01, sine_hz(hz)
  );
}
