//bgawk

attraction(0);

let h = [];
let l = [];
let th = [];
let tl = [];

for i in -10..10 {
    for j in -10..10 {
        let hue = (i+j).abs() * 18;
        let e = spawn(1).x(i*10).y(j*10).sides(4).h(hue).s(1).l(0.5).tail(0).sensor(true);
        let f = e.to_floats();
        h.push(f[0]);
        l.push(f[1]);
        let e = spawn(1).x(i*10).y(j*10).sides(4).h(hue).s(1).l(0.5).tail(0).sensor(true);
        let f = e.to_floats();
        th.push(f[0]);
        tl.push(f[1]);
    }
}

let i = spawn(2).x(35).y(-15).h(0).tail(0);
let j = spawn(2).x(-50).y(60).h(100).tail(0);

// update code
for n in 0..400 {
    let te = Entity::from_floats([th[n], tl[n]]);
    let e = Entity::from_floats([h[n], l[n]]);
    let x = e.x/100;
    let y = e.y/100;
    te.x(x * i.x + y * j.x);
    te.y(x * i.y + y * y * j.y);
}