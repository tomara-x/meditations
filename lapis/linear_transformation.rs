//bgawk https://github.com/tomara-x/bgawk
let h = [];
let l = [];
let th = [];
let tl = [];
//original
for i in 0..10 {
	for j in 0..10 {
		let e = spawn(1).x(i).y(j).h((i+j)*18);
		let f = e.to_floats();
		h.push(f[0]);
		l.push(f[1]);
	}
}
//transformed
for i in 0..10 {
	for j in 0..10 {
		let e = spawn(1).x(i*10).y(j*10).h((i+j)*18);
		let f = e.to_floats();
		th.push(f[0]);
		tl.push(f[1]);
	}
}
// basis vectors
let i = spawn(2).x(1).h(0);
let j = spawn(2).y(1).h(100);

//update code
for n in 0..100 {
	let te = Entity::from_floats([th[n], tl[n]]);
	let e = Entity::from_floats([h[n], l[n]]);
	// input vector is [e.x e.y]
	// multiplied by the matrix
	// [ ix, jx
	//   iy, jy ]
	te.x(e.x * i.x + e.y * j.x);
	te.y(e.x * i.y + e.y * j.y);
}