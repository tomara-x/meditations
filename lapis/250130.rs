let step = 0;

let seq = Sequencer::new(false, 1);
let be = seq.backend();
let clean = dcblock();
let filter = (pass() | dc(1729) | dc(1)) >> lowpass();
let verb = (pass() | pass()) & reverb_stereo(40, 3, 0.4);
let limit = limiter(0.1, 0.1);
let g = be >> clean >> filter >> limit >> pan(0) >> verb;
g.play();

let s1 = 0;

let w = Wave::load("/home/void/Music/hind swaraj/indianhomerule_06_gandhi_64kb.m4a");

let _ = spawn(2).x(63.90123).y(50).ry(2).rot(-0).mass(0).inertia(10).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(200).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(110.188156).y(-6.0360336).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(247.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 2., Fade::Smooth, 0, 0.05,
	wavech_at(w, 0, 1603680, 1630680, 1603680) >> dlowpass_hz(Atan(1.1), lerp(700, 3000, rnd1(s1)), 13)
)").code_f("");
let _ = spawn(2).x(10).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(22.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(20).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(45).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(30).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(67.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(40).y(-7.844803).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(90).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 1., Fade::Smooth, 0, 0.05,
	wavech_at(w, 0, 503680, 510680, 503680) >> dlowpass_hz(Atan(1.08), lerp(700, 3000, rnd1(s1)), 1)
)").code_f("");
let _ = spawn(2).x(120).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(270).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(0).y(-4.173088).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("s1 += 1;
seq.push_relative(
	0, 0.4, Fade::Smooth, 0, 0.05,
	bassdrum(1, 1000, 50) >> dlowpass_hz(Atan(1.1), lerp(700, 3000, rnd1(s1)), 14)
)").code_f("");
let _ = spawn(2).x(90).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(202.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(70).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(157.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(60).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(135).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(50).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(112.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(150).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(337.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(100).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(225).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(80).y(-9.1464).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(180).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 0.3, Fade::Smooth, 0, 0.05,
	wavech_at(w, 0, 7503680, 8503680) >> dlowpass_hz(Atan(1.08), lerp(700, 3000, rnd1(s1)), 48)
)").code_f("");
let _ = spawn(2).x(130).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(292.5).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("").code_f("");
let _ = spawn(2).x(140).y(0).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(315).s(1).l(0.19999999).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("seq.push_relative(
	0, 2., Fade::Smooth, 0, 0.05,
	wavech_at(w, 0, 1703680, 3416680) >> dlowpass_hz(Atan(1.1), lerp(700, 3000, rnd1(s1)), 14)
)").code_f("");
let _ = spawn(1).x(20).y(2.5).ry(1).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(1).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("x < step").code_i("$other.l($other.l+0.3)").code_f("$other.l($other.l-0.3)");
let _ = spawn(13).x(50).y(50).ry(13).rot(-2.0899715).mass(0).inertia(1).vx(0).vy(0).va(6.2831855).restitution(0.5).lindamp(0).angdamp(0).h(330).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("va < TAU").code_i("step = (step + 10)%160;").code_f("");
