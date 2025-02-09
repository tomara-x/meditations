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

let x = true;
"a" = {
	if x {
		"x" = {32;};
	} else {
		"x" = {};
	}
	x = !x;
};

//241025
let main = Net::new(0,2);

let mixer = main.push(join::<U8>());
let seq1 = Sequencer::new(false,1);
let be1 = main.push(seq1.backend());
let verb = main.push(split::<U2>() >> (multipass::<U2>() & reverb_stereo(60,10,0.4)));

main.connect(mixer, 0, verb, 0);
main.connect(be1, 0, mixer, 0);
main.connect_output(verb, 0, 0);
main.connect_output(verb, 1, 1);

main.play_backend();

seq1.push_relative(0, 1, Fade::Smooth, 0, 0, sine_hz(220));

let noise = main.push(pink() * 0.1);
main.connect(noise, 0, mixer, 1);
main.commit();

main.disconnect(mixer, 1);
main.commit();

//241026
let seq = Sequencer::new(false, 1);
let verb = multipass::<U2>() & reverb_stereo(40,10,0.4);
let g = seq.backend() >> split::<U2>() >> verb;

g.play();
let noise = white();
let r = [];

let mid_c = pow(2, -9/12) * 440;
let bpm = 115;
let beat = bpm / 4 / 60;

let scale = [1, 4, 7, 8, 14];
let divs = [1/2, 1/4, 1/8, 1/16, 1/32];

"shift+a" = {
for i in 0..16 {
	noise.tick([], r);
	let p2 = i*beat/4;
	let p3 = p2 + divs[lerp11(0,5,r[0])] * beat;
	let hz = semitone_ratio(scale[lerp11(0, 5, r[0])]) * mid_c;
	let _ = seq.push_relative(p2,p3,Fade::Smooth,0.01,0.01,sine_hz(hz)*0.5);
}
};

//241026
//use waves for loops
let main = Net::new(0,2);
let seq1 = Sequencer::new(false,1);
let be1 = main.push(seq1.backend());
let verb = main.push(split::<U2>() >> (multipass::<U2>() & reverb_stereo(40, 8, 0.4) * 0.7));

main.connect(be1, 0, verb, 0);
main.connect_output(verb, 0, 0);
main.connect_output(verb, 1, 1);

main.play_backend();

let mid_c = pow(2, -9/12) * 440;
let bpm = 115;
let bar = 60 / bpm;

let noise = white();
let r = [];

"shift+q" = {
for i in 0..8 {
	noise.tick([], r);
	let _ = seq1.push_relative(i*bar/4, i*bar/4+bar/4, Fade::Smooth, 0, 0, snaredrum(r[0]*1000,1));
}
};

"shift+w" = {
for i in 0..8 {
	noise.tick([], r);
	let _ = seq1.push_relative(i*bar/8, i*bar/8+bar/8, Fade::Smooth, 0, 0, snaredrum(r[0]*3000,0.9));
}
};

main.connect_output(be1, 0, 0);
main.connect_output(be1, 0, 1);
main.commit();

//241103
let wave = Wave::load("/home/void/Music/230713r2.m4a");
let sr = 44100;
let lp = lowpass_hz(1000, 2);
let w1 = wavech_at(wave, 0, 24*sr, 25.2*sr, 24*sr) >> lp.clone();
let w2 = wavech_at(wave, 0, 2.3*sr, 7.5*sr, 2.3*sr) >> lp.clone();
let w3 = wavech_at(wave, 0, 3.5*sr, 5.3*sr, 3.5*sr) >> lp.clone();
((w1.clone() + w2.clone() + w3.clone()) * 0.5 >> pan(0)).play();
