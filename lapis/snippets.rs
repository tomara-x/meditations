// lapis

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


// 250124
let step = 0;
let _ = spawn(2).x(64).y(20).ry(2).rot(-0).mass(0).inertia(10).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(200).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(13).x(50).y(20).ry(13).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(330).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("va < 115/60*TAU").code_i("step = (step + 10)%160;").code_f("");
let _ = spawn(1).x(90).y(0).z(1).ry(1).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(1).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("x < step").code_i("$other.l($other.l+0.3)").code_f("$other.l($other.l-0.3)");

for i in 0..16 {
    let _ = spawn(2).x(i*10).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(i/16*360).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
}

// 250119
let _ = spawn(23.737732).x(23.214722).y(7.617689).ry(23.737732).rot(-0).mass(13375.735).inertia(13375.735).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(322.17392).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(1).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.462032).x(-63.427048).y(176.31296).ry(23.462032).rot(-0).mass(12915.074).inertia(12915.074).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(195.91304).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.263206).x(32.064972).y(-180.89616).ry(23.263206).rot(-0).mass(12589.507).inertia(12589.507).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(282.78262).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(7).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.26918).x(-95.27477).y(-98.14672).ry(23.26918).rot(-0).mass(12599.209).inertia(12599.209).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(2).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(22.828176).x(237.96188).y(81.65545).ry(22.828176).rot(-0).mass(11896.349).inertia(11896.349).vx(-0.009270461).vy(-0.0046439096).va(0).restitution(0.5).lindamp(0).angdamp(0).h(19.30435).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(3).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.72709).x(-133.2032).y(51.917564).ry(21.72709).rot(-0).mass(10256.631).inertia(10256.631).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(48).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(4).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.380722).x(82.296524).y(-96.98157).ry(21.380722).rot(-0).mass(9773.883).inertia(9773.883).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(102.521736).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(5).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.143606).x(88.14186).y(203.13376).ry(21.143606).rot(-0).mass(9452.293).inertia(9452.293).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(226.1739).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(6).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(6.302681).x(22.068596).y(-26.493317).ry(6.302681).rot(-1.951075).mass(250.36636).inertia(250.36636).vx(68.52816).vy(34.328236).va(-15.802525).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(90).layer(3).dynamic(true).sensor(false).links("").code_i("$id.layer(($other.layer + 1) % 8).h($other.h);").code_f("$id.vx(0).vy(0);");


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
