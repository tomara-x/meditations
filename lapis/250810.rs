"quiet" = true;

let seq = Sequencer::new(false, 1);
let g = seq.backend() >> pan(0);
g.play();
let s1 = 0;
let s2 = 0;

let q = quantizer([0,1,4,5,7,10,11, 12]);
let o = [];

let midc = 440 * exp2(-9/12);

let e1 = spawn(66.440674).x(-171.24123).y(-156.56258).z(0).ry(66.440674).rot(1.343667).mass(293293.28).inertia(293293.28).vx(-0.6317834).vy(0.6240093).va(-0.25652564).restitution(0.95).lindamp(0).angdamp(0).h(0).s(1).l(1).a(1).sides(32).cmx(0).cmy(0).friction(0).tail(90).layer(0).dynamic(true).sensor(false).links("").code_i("e1.h(e2.h);
e2.h(e3.h);
e3.h(e1.h);
e1.l(e2.l);
e2.l(e3.l);
e3.l(e1.l);
").code_f("");
let e2 = spawn(7.056332).x(-98.76889).y(-155.83301).z(0).ry(7.056332).rot(-0.6896148).mass(351.34766).inertia(351.34766).vx(296.98438).vy(101.42469).va(-149.56653).restitution(0.95).lindamp(0).angdamp(0).h(200).s(1).l(0.6352941).a(1).sides(32).cmx(0).cmy(0).friction(0.5).tail(90).layer(0).dynamic(true).sensor(false).links("").code_i("s2 += 1;
let i = lerp(0, 36, rnd1(s2));
q.tick([i], o);

seq.push_relative(
  0,0.1,
  Fade::Smooth,
  0.01, 0.01,
  organ_hz(midc * exp2(o[0]/12) / 2) * 0.1
);").code_f("");
let e3 = spawn(6.407837).x(-133.26123).y(-60.232674).z(0).ry(6.407837).rot(2.4955473).mass(263.1082).inertia(263.1082).vx(-197.46211).vy(-636.4142).va(439.83603).restitution(0.95).lindamp(0).angdamp(0).h(0.71428573).s(1).l(0.8352941).a(1).sides(32).cmx(0).cmy(0).friction(0).tail(90).layer(0).dynamic(true).sensor(false).links("").code_i("s1 += 1;
let i = lerp(0, 36, rnd1(s1));
q.tick([i], o);

seq.push_relative(
  0,0.1,
  Fade::Smooth,
  0.01, 0.01,
  sine_hz(midc * exp2(o[0]/12)) * 0.1
);").code_f("");
