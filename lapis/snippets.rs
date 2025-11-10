let nmin = [0,2,3,5,7,8,10];
let mmin = [0,2,3,5,7,9,11];
let hmin = [0,2,3,5,7,8,11];
let major = [0,2,4,5,7,9,11];
let blues_6 = [0,3,5,6,7,10];
let blues_9 = [0,2,3,4,5,6,7,9,10];
let ukrainian = [0,2,3,6,7,9,10];
let double_harm = [0,1,4,5,7,8,11];
let wt = [0,2,4,6,8,10];
let mothra6 = [0,6,12,18,24,30];
let mode_2 = [0,6,12,18,24,25];
let mode_3 = [0,6,12,18,19,25];
let mode_4 = [0,6,12,13,19,25];
let mode_5 = [0,6,7,13,19,25];
let mode_6 = [0,1,7,13,19,25];
let mothra11 = [0,1,6,7,12,13,18,19,24,25,30];

let scale = nmin;
let slen = scale.len();

let bpm = 115;
let beat = 60 / bpm;

let midc = 440 * exp2(-9/12);

let seq = Sequencer::new(false, 1);
let be = seq.backend();
let clean = dcblock();
let filter = (pass() | dc(1729) | dc(1)) >> lowpass();
let verb = (pass() | pass()) & reverb_stereo(40, 3, 0.4);
let limit = limiter(0.1, 0.1);
let g = be >> clean >> filter >> limit >> pan(0) >> verb;
g.play();

//let note = (scale[n%slen] + NOTE_OFFSET)/12;
//let oct = (n/slen).floor() % MAX_OCT + OCT_OFFSET;
//let f = midc * exp2(note + oct);

let f = midc * exp2(SEMI/12 + OCT);

seq.push_relative(
    0, 0.4, Fade::Smooth, 0.01, 0.01,
    sine_hz(f)
);

let meditation = "let m = $;
let times = [0.1, 1, 1];
for _ in 0..m {
	times.insert(1, $*60);
	times.insert(2, 0.1);
}
let med = unsteady(times, false) >> step(sine_hz(1760), dc(0)) >> pan(0);
med.play();";
quiet_eval(format(meditation, 1, 1));

// blue
let t = [0, 1, 2, 3, 5, 3, 7, 2];
let x = [0, 0, 0, 0, 0, 0, 0, 0]; let xm = 0;
let y = [0, 0, 0, 0, 0, 0, 0, 0]; let ym = 0;
let z = [0, 0, 0, 0, 0, 0, 0, 0]; let zm = 0;
let s = 0;
let n = 0;
s = (s + 1) % slen;
n = t[s] + x[s] * xm + y[s] * ym + z[s] * zm;

// borb
let seq = Sequencer::new(false, 1);
let g = seq.backend() >> pan(0);
g.play();
for i in 0..20 {
	let r = rnd1(i);
	seq.push_relative(
		r*0.1, r*0.1+0.02, Fade::Smooth, 0.001, 0.001,
		sine_hz(lerp(r, 400, 2000))
	);
}
//sine_hz(lerp(r, 470, 3000))
//sine_hz(lerp(r, 470, 8000))
for i in 0..10 {
	let r = rnd1(i);
	seq.push_relative(
		r*0.2, r*0.4+0.02, Fade::Smooth, 0.001, 0.001,
		sine_hz(lerp(r, 470, 8000))
	);
}

