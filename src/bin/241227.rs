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
let verb = (pass() | pass()) & reverb_stereo(40, 3, 0.4);
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

// 250115
let t = [0, 6, 8, 16, 5, 14, 7, 2];
let seed = 0;
t = [4, 1, 2, 3, 5, 14, 19, 13];
// code_i
seed += 1;
let l = rnd1(seed) * 8;
$id.layer(l).h($other.h);
n = t[$other.layer];
let note = (scale[n%slen] + 0)/12;
let oct = (n/slen).floor() % 2 +  2;
let f = midc * exp2(note + oct);
seq.push_relative(
    0, 0.7, Fade::Smooth, 0, 0.01,
    brown() >> pluck(f, 14.7, 0.4)*0.4
);
// code_f
$id.vx(0).vy(0);

// 250118
let _ = spawn(6.302681).x(-37.645927).y(169.33961).ry(6.302681).rot(-3.1415927).mass(250.36636).inertia(250.36636).vx(-3.7480319).vy(1.265878).va(-0.12979266).restitution(0.5).lindamp(0).angdamp(0).h(331.56522).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(90).layer(0).dynamic(true);
let _ = spawn(23.462032).x(-63.427048).y(176.31296).ry(23.462032).rot(-0).mass(12915.074).inertia(12915.074).vx(0.09049184).vy(-0.024476461).va(0).restitution(0.5).lindamp(0).angdamp(0).h(195.91304).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false);
let _ = spawn(23.737732).x(49.79576).y(49.62507).ry(23.737732).rot(-0).mass(13375.735).inertia(13375.735).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(322.17392).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(1).dynamic(false);
let _ = spawn(23.26918).x(-95.27477).y(-98.14672).ry(23.26918).rot(-0).mass(12599.209).inertia(12599.209).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(2).dynamic(false);
let _ = spawn(22.828176).x(237.96188).y(81.65545).ry(22.828176).rot(-0).mass(11896.349).inertia(11896.349).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(19.30435).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(3).dynamic(false);
let _ = spawn(21.72709).x(-133.2032).y(51.917564).ry(21.72709).rot(-0).mass(10256.631).inertia(10256.631).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(48).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(4).dynamic(false);
let _ = spawn(21.380722).x(82.296524).y(-96.98157).ry(21.380722).rot(-0).mass(9773.883).inertia(9773.883).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(102.521736).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(5).dynamic(false);
let _ = spawn(21.143606).x(88.14186).y(203.13376).ry(21.143606).rot(-0).mass(9452.293).inertia(9452.293).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(226.1739).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(6).dynamic(false);
let _ = spawn(23.263206).x(32.064972).y(-180.89616).ry(23.263206).rot(-0).mass(12589.507).inertia(12589.507).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(282.78262).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(7).dynamic(false);


let _ = spawn(6.302681).x(-1.4877834).y(-128.94563).ry(6.302681).rot(0.22135396).mass(250.36636).inertia(250.36636).vx(86.14523).vy(-133.38066).va(16.596878).restitution(0.5).lindamp(0).angdamp(0).h(48).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(90).layer(7).dynamic(true).sensor(false).links("").code_i("seed += 1;
let l = rnd1(seed) * 8;
$id.layer(l).h($other.h);").code_f("$id.vx(0).vy(0);");
