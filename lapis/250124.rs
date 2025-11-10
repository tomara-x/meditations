//bgawk

"quiet" = true;
let step = 0;
let cutoff = shared(1729);
let q = shared(1);

let seq = Sequencer::new(false, 1);
let be = seq.backend();
let verb = (pass() | pass()) & reverb_stereo(40, 3, 0.4);
let limit = limiter(0.1,0.1);
let filter = (pass() | var(cutoff) | var(q)) >> lowpass();
let g = be >> dcblock() >> filter >> limit >> pan(0) >> verb;
g.play();


let _ = spawn(2).x(0).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 0.7, Fade::Smooth, 0, 0.01,
	bassdrum(0.7, 400, 70)
)").code_f("");
let _ = spawn(2).x(83.713776).y(270.13403).ry(2).rot(-0).mass(0).inertia(10).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(200).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("step = (step + 10)%160;").code_f("");
let _ = spawn(2).x(10).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(22.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(130).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(292.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(30).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(67.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(100).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(225).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(50).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(112.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(60).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(135).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(20).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(45).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(150).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(337.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(70).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(157.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(120.15323).y(233.51454).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(270).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0.01, 0.9, Fade::Smooth, 0, 0.01,
	snaredrum(0.7, 4) * brown() * 4
);
seq.push_relative(
	0.06, 0.9, Fade::Smooth, 0, 0.01,
	sine_hz(750)* 0.04 + brown()
);").code_f("");
let _ = spawn(2).x(80.379715).y(234.22649).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(180).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 0.7, Fade::Smooth, 0, 0.01,
	bassdrum(0.7, 400, 70) * brown() * 8
)").code_f("");
let _ = spawn(2).x(40.022343).y(233.71272).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(90).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0.6, 0.9, Fade::Smooth, 0, 0.01,
	snaredrum(0.7, 4) * brown() * 2
);
seq.push_relative(
	0.0, 0.9, Fade::Smooth, 0, 0.1,
	brown()
);").code_f("");
let _ = spawn(2).x(110.06702).y(233.0873).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(247.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0.0, 0.1, Fade::Smooth, 0, 0.1,
	pink()* 0.6
);
seq.push_relative(
	0.3, 0.5, Fade::Smooth, 0, 0.1,
	pink()* 0.3
);").code_f("");
let _ = spawn(2).x(140).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(315).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(90.089355).y(233.55638).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(202.5).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0.5, 0.7, Fade::Smooth, 0, 0.01,
	bassdrum(0.7, 400, 70) * brown() * 2
)").code_f("");
let _ = spawn(1).x(30).y(237.81296).ry(1).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(1).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("x < step").code_i("$other.l($other.l+0.3)").code_f("$other.l($other.l-0.3)");
let _ = spawn(13).x(69.81253).y(270.13403).ry(13).rot(2.96603).mass(0).inertia(1).vx(0).vy(0).va(6.2831855).restitution(0.5).lindamp(0).angdamp(0).h(330).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("va < TAU").code_i("").code_f("");

// from snippets
let step = 0;
let _ = spawn(2).x(64).y(20).ry(2).rot(-0).mass(0).inertia(10).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(200).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(13).x(50).y(20).ry(13).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(330).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("va < 115/60*TAU").code_i("step = (step + 10)%160;").code_f("");
let _ = spawn(1).x(90).y(0).z(1).ry(1).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(1).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("x < step").code_i("$other.l($other.l+0.3)").code_f("$other.l($other.l-0.3)");

for i in 0..16 {
    let _ = spawn(2).x(i*10).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(i/16*360).s(1).l(0.2).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
}

