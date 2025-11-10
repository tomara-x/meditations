// bgawk
"quiet" = true;

let step = 0;

let scale = [0,2,3,5,7,8,10];
let bpm = 86;
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

let s = 0;
let r = 0;

let _ = spawn(13).x(50).y(20).z(0).ry(13).rot(-1.53912).mass(0).inertia(1).vx(0).vy(0).va(9.005898).restitution(0.5).lindamp(0).angdamp(0).h(330).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("va < 86/60*TAU").code_i("step = (step + 10)%160;").code_f("");
let _ = spawn(1).x(30).y(0).z(1).ry(1).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(1).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("x < step").code_i("$other.l($other.l+0.3)").code_f("$other.l($other.l-0.3)");
let _ = spawn(2).x(64).y(20).z(0).ry(2).rot(-0).mass(0).inertia(10).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(200).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(150).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(337.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(10).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(22.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(20).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(45).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(30).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(67.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(130).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(292.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(50).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(112.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(60).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(135).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(70).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(157.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(140).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(315).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(90).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(202.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(100).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(225).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(110).y(0).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(247.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(40.20092).y(-0.3076081).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(90).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 0.7,
	Fade::Smooth,
	0, 0,
	snaredrum(1, 80)
);

for i in 0..4 {
	s += 1;
	r = rnd1(s);
	let f = midc * exp2(scale[r*8]/12);
	seq.push_relative(
		0, 0.7,
		Fade::Smooth,
		0.01, 0.01,
		sine_hz(f)
	);
}").code_f("");
let _ = spawn(2).x(80.02239).y(-0.1446476).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(180).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 0.3,
	Fade::Smooth,
	0, 0,
	bassdrum(1, 400, 80)
);").code_f("");
let _ = spawn(2).x(0.1306305).y(0.207141).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 0.3,
	Fade::Smooth,
	0, 0.01,
	bassdrum(1, 400, 80)
);").code_f("");
let _ = spawn(2).x(119.94588).y(0.12015447).z(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(270).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 0.7,
	Fade::Smooth,
	0, 0,
	snaredrum(2, 60)
);
for i in 0..3 {
	s += 1;
	r = rnd1(s);
	let f = midc * exp2(scale[r*8]/12);
	seq.push_relative(
		i*beat/3, (i+1)*beat/3,
		Fade::Smooth,
		0.01, 0.01,
		sine_hz(f) * 0.5
	);
}").code_f("");
