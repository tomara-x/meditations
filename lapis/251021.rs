let seq = Sequencer::new(false, 1);
let g = seq.backend() >> pan(0);
g.play();

let t = 1;

let subsubsub2 = unsteady_no_reset([t/4/3], true)
>> step(
    cymbal(1) * 0.1,
);

let subsubsub1 = unsteady_no_reset([t/8], true)
>> step(
    cymbal(1) * 0.1,
    soft_saw_hz(220),
);

let subsub = unsteady_no_reset([t/4], true)
>> step(
    cymbal(1) * 0.1,
    dc(0),
    subsubsub1,
    subsubsub2,
);

let sub1 = unsteady_no_reset([t/4], true)
>> step(
    bassdrum(0.5, 1000, 80),
    bassdrum(0.5, 1000, 80),
    cymbal(1) * 0.1,
    bassdrum(0.5, 1000, 80),
    snaredrum(0, 0.4),
    bassdrum(0.5, 1000, 80),
    subsub,
);

let sub2 = unsteady_no_reset([t/2], true)
>> step(
    organ_hz(1100) * ahr(0.002, 0, 0.1),
    bassdrum(0.5, 4000, 80),
);

let pat = unsteady([t, t/2], true)
>> step(
    sub1,
    sub2,
);

let _ = seq.push_relative(0, 24, Fade::Smooth, 0, 0, pat);
