use std::collections::HashMap;

const MAX_TRIES: u32     = 100_000;
const MAX_REPS: u32      = 10;
const DEFAULT_ALPHA: f64 = 0.001;
const DEFAULT_DECAY: f64 = 0.001;

#[derive(Debug)]
pub struct Options {
    pub max_tries: Option<u32>,
    pub max_reps:  Option<u32>,
    pub alpha:     Option<f64>,
    pub decay:     Option<f64>
}

#[derive(Clone, Copy)]
pub struct Function<'f> {
    pub base: &'f dyn Fn(f64) -> f64,
    pub derivate: Option<&'f dyn Fn(f64) -> f64>,
}

pub fn gradient_descent(start: f64, func: Function, opts: Option<Options>) -> (i32, u32) {
    let mut opts: Options = match opts {
        Some(opts) => opts,
        None => Options{
            max_tries: Some(MAX_TRIES),
            max_reps: Some(MAX_REPS),
            alpha: Some(DEFAULT_ALPHA),
            decay: Some(DEFAULT_DECAY)
        }
    };
    opts.max_tries = opts.max_tries.or(Some(MAX_TRIES));
    opts.max_reps = opts.max_reps.or(Some(MAX_REPS));
    opts.alpha = opts.alpha.or(Some(DEFAULT_ALPHA));
    opts.decay = opts.decay.or(Some(DEFAULT_DECAY));
    println!("{:?}", opts);

    let mut reps: HashMap<String, u32> = HashMap::new();
    let mut x: f64 = start;
    for epoch in 0..opts.max_tries.unwrap() {
        // println!("{}, {}, {}", epoch, x, func.derivate.unwrap()(x));
        x = x - opts.alpha.unwrap() * func.derivate.unwrap()(x);

        *reps.entry(format!("{}", x)).or_insert(0) += 1;
        if reps[&format!("{}", x)] > opts.max_reps.unwrap() {
            return (x as i32, epoch)
        }
    }

    (x as i32, opts.max_tries.unwrap())
}

pub fn inf_nan_or(x: f64, or: f64) -> f64 {
    match x.is_infinite() || x.is_nan() {
        true => or,
        false => x
    }
}

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

pub fn median(list: &[i32]) -> f64 {
    let len = list.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&list[(mid - 1)..(mid + 1)])
    } else {
        f64::from(list[mid])
    }
}