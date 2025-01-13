// lapis
let bpm = 115;
let bar = 60 / bpm;

let midc = 440 * exp2(-9/12);

let nmin = [0,2,3,5,7,8,10];
let mmin = [0,2,3,5,7,9,11];
let hmin = [0,2,3,5,7,8,11];
let major = [0,2,4,5,7,9,11];
let blues_6 = [0,3,5,6,7,10];
let blues_9 = [0,2,3,4,5,6,7,9,10];
let ukrainian = [0,2,3,6,7,9,10];
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

let seq = Sequencer::new(false, 1);
let be = seq.backend();
let verb = reverb_stereo(20, 3, 0.4);
let g = be >> split::<U2>() >> verb;
g.play();

let t = [0, 1, 2, 3, 5, 3, 7, 2];
let x = [0, 0, 0, 0, 0, 0, 0, 0]; let xm = 0;
let y = [0, 0, 0, 0, 0, 0, 0, 0]; let ym = 0;
let z = [0, 0, 0, 0, 0, 0, 0, 0]; let zm = 0;
let s = 0;
let n = 0;
s = (s + 1) % slen;
n = t[s] + x[s] * xm + y[s] * ym + z[s] * zm;

let note = (scale[n%slen] + NOTE_OFFSET)/12;
let oct = (n/slen).floor() % MAX_OCT + OCT_OFFSET;
let f = midc * exp2(note + oct);

seq.push_relative(
    0, 0.4, Fade::Smooth, 0.01, 0.01,
    sine_hz(f)
);

// 250112
let midc = 440 * exp2(-9/12);
let scale = [0,2,3,5,7,8,10];
let slen = scale.len();
let seq = Sequencer::new(false, 1);
let be = seq.backend();
let verb = reverb_stereo(40, 3, 0.4);
let limit = limiter(0.1, 0.1);
let g = be >> limit >> pan(0) >> verb;
g.play();
let t = [0, 6, 5, 3, 5, 14, 7, 2];
let n = 0;

let l = ($other.layer + 3) % 8;
$id.layer(l).h($other.h).vx(0).vy(0).va(0);

n = t[$other.layer];
let note = (scale[n%slen] + 0)/12;
let oct = (n/slen).floor() % 4 - 2;
let f = midc * exp2(note + oct);

seq.push_relative(
    0, 0.2, Fade::Smooth, 0.01, 0.01,
    organ_hz(f)
);
