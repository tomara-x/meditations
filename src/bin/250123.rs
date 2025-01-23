// bgawk

let midc = 440 * exp2(-9/12);
let scale = [0,2,3,5,7,8,10];
let slen = scale.len();
let t = [0, 6, 5, 3, 5, 14, 7, 2];
let n = 0;

let cutoff = shared(0);
let q = shared(0);

let seq = Sequencer::new(false, 1);
let be = seq.backend();
let verb = (pass() | pass()) & reverb_stereo(40, 3, 0.4);
let limit = limiter(0.1,0.1);
let clean = dcblock();
let filter = (pass() | var(cutoff) | var(q)) >> lowpass();
let g = be >> clean >> filter >> limit >> pan(0) >> verb;
g.play();


let _ = spawn(23.462032).x(-63.427048).y(174.35197).ry(23.462032).rot(-0).mass(12915.074).inertia(12915.074).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(195.91304).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.26918).x(-115.82602).y(-178.85164).ry(23.26918).rot(-0).mass(12599.209).inertia(12599.209).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(2).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.263206).x(32.064972).y(-180.89616).ry(23.263206).rot(-0).mass(12589.507).inertia(12589.507).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(282.78262).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(7).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.380722).x(75.23686).y(-84.431274).ry(21.380722).rot(-0).mass(9773.883).inertia(9773.883).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(102.521736).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(5).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(6.302681).x(86.582306).y(88.25115).ry(6.302681).rot(-0.644752).mass(250.36636).inertia(250.36636).vx(323.02872).vy(414.5267).va(-43.46025).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(90).layer(3).dynamic(true).sensor(false).links("").code_i("$id.layer(($other.layer + 1) % 8).h($other.h);

n = t[$other.layer];
let note = (scale[n%slen] + 0)/12;
let oct = (n/slen).floor() % 2 - 0;
let f = midc * exp2(note + oct);
seq.push_relative(
    0, 1.7, Fade::Smooth, 0, 0.01,
    brown() >> pluck(f, 28.7, 0.9)*0.4
);").code_f("$id.vx(0).vy(0);");
let _ = spawn(11.411555).x(58.396027).y(-285.11237).ry(11.411555).rot(-0).mass(1486.0537).inertia(1486.0537).vx(0.52057683).vy(-0.11986039).va(0).restitution(0.5).lindamp(0).angdamp(0).h(321.76166).s(1).l(0.6215686).a(1).sides(3).cmx(0).cmy(0).friction(0.5).tail(0).layer(10).dynamic(false).sensor(false).links("x > q").code_i("").code_f("");
let _ = spawn(22.828176).x(143.06659).y(160.73465).ry(22.828176).rot(-0).mass(11896.349).inertia(11896.349).vx(-0.20096458).vy(-0.25788796).va(0).restitution(0.5).lindamp(0).angdamp(0).h(19.30435).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(3).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.143606).x(88.14186).y(203.13376).ry(21.143606).rot(-0).mass(9452.293).inertia(9452.293).vx(-0.010964482).vy(-0.11121501).va(0).restitution(0.5).lindamp(0).angdamp(0).h(226.1739).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(6).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.737732).x(23.214722).y(7.617689).ry(23.737732).rot(-0).mass(13375.735).inertia(13375.735).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(322.17392).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(1).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.72709).x(-133.2032).y(51.917564).ry(21.72709).rot(-0).mass(10256.631).inertia(10256.631).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(48).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(4).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(6.302681).x(61.765324).y(-64.4089).ry(6.302681).rot(3.1178942).mass(250.36636).inertia(250.36636).vx(6.8929152).vy(69.916245).va(190.45651).restitution(0.5).lindamp(0).angdamp(0).h(102.521736).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(90).layer(6).dynamic(true).sensor(false).links("").code_i("$id.layer(($other.layer + 1) % 8).h($other.h);

n = t[$other.layer];
let note = (scale[n%slen] + 0)/12;
let oct = (n/slen).floor() % 2 + 1;
let f = midc * exp2(note + oct);
seq.push_relative(
    0, 0.7, Fade::Smooth, 0, 0.01,
    pink() >> pluck(f, 199.7, 1.9)*0.4
);").code_f("$id.vx(0).vy(0);");
let _ = spawn(12.30682).x(466.43365).y(-379.06113).ry(12.30682).rot(-0).mass(1863.964).inertia(1863.964).vx(-0.41503224).vy(0.09555924).va(0).restitution(0.5).lindamp(0).angdamp(0).h(321.76166).s(1).l(0.6215686).a(1).sides(3).cmx(0).cmy(0).friction(0.5).tail(0).layer(10).dynamic(false).sensor(false).links("x > cutoff").code_i("").code_f("");


// round 2
let midc = 440 * exp2(-9/12);
let scale = [0,2,3,5,7,8,10];
let slen = scale.len();
let t = [0, 6, 5, 3, 5, 14, 7, 2];
let n = 0;

let cutoff = shared(0);
let q = shared(0);

let seq = Sequencer::new(false, 1);
let be = seq.backend();
let verb = (pass() | pass()) & reverb_stereo(40, 3, 0.4);
let limit = limiter(0.1,0.1);
let clean = dcblock();
let filter = (pass() | var(cutoff) | var(q)) >> lowpass();
let g = be >> clean >> filter >> limit >> pan(0) >> verb;
g.play();
let x = [1,0,3,1,2,0,0,2];
let xm = 0;

let _ = spawn(6.302681).x(-71.133736).y(147.10385).ry(6.302681).rot(0.3612188).mass(250.36636).inertia(250.36636).vx(-2.3431575).vy(-6.264387).va(0.23398614).restitution(0.5).lindamp(0).angdamp(0).h(195.91304).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(90).layer(2).dynamic(true).sensor(false).links("").code_i("let l = ($other.layer + 1) % 8;
$id.layer(l).h($other.h);

n = t[l] + x[l] * xm;
let note = (scale[n%slen] + 0)/12;
let oct = (n/slen).floor() % 3 - 1;
let f = midc * exp2(note + oct);
seq.push_relative(
    0, 1.7, Fade::Smooth, 0, 0.01,
    ((sine_hz(5) * $id.rot * 2 + dc(f)) >> sine())*0.4
);").code_f("$id.vx(0).vy(0);");
let _ = spawn(23.26918).x(-115.82602).y(-178.85164).ry(23.26918).rot(-0).mass(12599.209).inertia(12599.209).vx(0.0018262081).vy(0.01500588).va(0).restitution(0.5).lindamp(0).angdamp(0).h(0).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(2).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.263206).x(32.064972).y(-180.89616).ry(23.263206).rot(-0).mass(12589.507).inertia(12589.507).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(282.78262).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(7).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.380722).x(75.23686).y(-84.431274).ry(21.380722).rot(-0).mass(9773.883).inertia(9773.883).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(102.521736).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(5).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.462032).x(-63.427048).y(174.35197).ry(23.462032).rot(-0).mass(12915.074).inertia(12915.074).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(195.91304).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(false).links("").code_i("xm += 1;").code_f("");
let _ = spawn(12.30682).x(555.7151).y(-382.0249).ry(12.30682).rot(-0).mass(1863.964).inertia(1863.964).vx(-0.025996031).vy(0.0045416853).va(0).restitution(0.5).lindamp(0).angdamp(0).h(321.76166).s(1).l(0.6215686).a(1).sides(3).cmx(0).cmy(0).friction(0.5).tail(0).layer(10).dynamic(false).sensor(false).links("x > cutoff").code_i("").code_f("");
let _ = spawn(22.828176).x(143.06659).y(160.73465).ry(22.828176).rot(-0).mass(11896.349).inertia(11896.349).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(19.30435).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(3).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.143606).x(88.14186).y(203.13376).ry(21.143606).rot(-0).mass(9452.293).inertia(9452.293).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(226.1739).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(6).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(23.737732).x(23.214722).y(7.617689).ry(23.737732).rot(-0).mass(13375.735).inertia(13375.735).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(322.17392).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(1).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(21.72709).x(-133.2032).y(51.917564).ry(21.72709).rot(-0).mass(10256.631).inertia(10256.631).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(48).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(0).layer(4).dynamic(false).sensor(false).links("").code_i("").code_f("");
let _ = spawn(11.411555).x(1).y(-285.11237).ry(11.411555).rot(-0).mass(1486.0537).inertia(1486.0537).vx(0.03260694).vy(-0.0056966566).va(0).restitution(0.5).lindamp(0).angdamp(0).h(321.76166).s(1).l(0.6215686).a(1).sides(3).cmx(0).cmy(0).friction(0.5).tail(0).layer(10).dynamic(false).sensor(false).links("x > q").code_i("").code_f("");
let _ = spawn(6.302681).x(-80.633064).y(152.72742).ry(6.302681).rot(-2.7846558).mass(250.36636).inertia(250.36636).vx(-3.3571684).vy(-8.410456).va(0.20114808).restitution(0.5).lindamp(0).angdamp(0).h(195.91304).s(1).l(0.5490196).a(1).sides(6).cmx(0).cmy(0).friction(0.5).tail(90).layer(2).dynamic(true).sensor(false).links("").code_i("let l = ($other.layer + 1) % 8;
$id.layer(l).h($other.h);

n = t[l] + x[l] * xm;
let note = (scale[n%slen] + 0)/12;
let oct = (n/slen).floor() % 2 + 1;
let f = midc * exp2(note + oct);
seq.push_relative(
    0, 0.7, Fade::Smooth, 0, 0.01,
    ((sine_hz(4) * $id.rot + dc(f)) >> sine())*0.4
);").code_f("$id.vx(0).vy(0);");

// mara?
// disable attraction

let step = 0;
let _ = spawn(2).x(63.90123).y(270).ry(2).rot(-0).mass(0).inertia(10).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(330).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("step = (step + 10)%160;").code_f("");
let _ = spawn(13).x(50).y(270).ry(13).rot(1.3952327).mass(0).inertia(1).vx(0).vy(0).va(1.5707964).restitution(0.5).lindamp(0).angdamp(0).h(276).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("va < TAU/4").code_i("").code_f("");
let _ = spawn(1).x(90).y(237.81296).ry(1).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(200).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(true).sensor(false).links("x < step").code_i("").code_f("");

for i in 0..16 {
let _ = spawn(2).x(i*10).y(235.99106).ry(2).rot(-0).mass(0).inertia(1).vx(0).vy(0).va(0).restitution(0.5).lindamp(0).angdamp(0).h(27.529411).s(1).l(0.5).a(1).sides(4).cmx(0).cmy(0).friction(0.5).tail(0).layer(0).dynamic(false).sensor(true).links("").code_i("$id.l(1.5);").code_f("$id.l(0.5);");
}
