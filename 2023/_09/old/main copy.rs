use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::read_to_string;
use plotters::prelude::*;
use compute::predict::PolynomialRegressor;

mod predictors;

const N: i32 = 10_000;

// TODO: make predict trait that has predict(f64) -> f64 func
//       For that convert both polyreg and lagrange into struct that keep the stuff
// TODO: export to own rust file, compose part_one, part_two files
// TODO: understand how PolynomialRegressor works with fitting(?), (opt: make one example by hand)
//       see: https://en.wikipedia.org/wiki/Polynomial_regression
// TODO: get into newton polynoms (and check if they compare to lagrange after all)
// TODO: make cubic splines for good measure (;))
// TODO: finish exercise (finally)
// TODO: read into Vandermonde matrix (as it's used in the PolynomialRegressor as well)

fn main() {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 1); // first is always the filename
    let input_file: String = args.pop().map_or_else(|| panic!("No arguments given"), |p| p);
    assert!(Path::new(&input_file).exists());
    
    let histories: Vec<Vec<i64>> = match parse(&input_file) {
        Ok(v) => v,
        Err(e) => panic!("{e}")
    };
    let width = histories.get(0).unwrap().len();
    let obs_lengths: i32 = histories.iter().map(|n| n.len() as i32).sum();
    let exp_lengths: i32 = (width * histories.len()) as i32; 
    assert_eq!(obs_lengths, exp_lengths);

    // assert linspace
    let (indices_1, range_1) = get_linspace(&vec![0.0, 6.0], 6);
    assert_eq!(indices_1, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert_eq!(range_1, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let (indices_1, range_1) = get_linspace(&vec![1.0, 6.0], 10);
    assert_eq!(indices_1, vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0]);
    assert_eq!(range_1, vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0]);

    for (idx, history) in histories.iter().enumerate() {
        // original x/y
        let x: Vec<f64> = (0 .. history.len()).map(|n| n as f64).collect();
        let y: Vec<f64> = history.iter().map(|n| *n as f64).collect();
        let mut predictors: HashMap<String, Vec<f64>> = HashMap::new();
        
        // iterate regressors
        let (poly_deg, poly_y) = get_polynomial_regressor(&x, &y);
        let lagrange_y = get_lagrange_interpol_polynom(&x, &y);
        
        let base_length = poly_y.iter().len();
        assert!(vec![poly_y.iter().len(), lagrange_y.len()]
            .iter()
            .map(|n| n / base_length).all(|n| n == 1));

        predictors.insert(format!("poly_reg {poly_deg}th"), poly_y);
        predictors.insert(format!("lagrange interpol poly"), lagrange_y);

        if let Err(e) = plot(idx, &x, history, &predictors) {
            panic!("nooo {e}");
        }
        println!("{idx} | done {:?}", history);
        return
    }
}

fn parse(input_file: &String) -> Result<Vec<Vec<i64>>, Box<dyn std::error::Error>> {
    let body = read_to_string(input_file)?;
    let lines = body.lines();
    let numbers = lines.map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|numbers| numbers
            .into_iter()
            .map(|n| n.parse::<i64>().map_err(|x| x.to_string()))
            .collect::<Result<Vec<i64>, String>>())
        .collect::<Vec<Result<Vec<i64>, String>>>();
    
    let mut histories = Vec::new();
    for number in numbers {
        histories.push(number?);
    }
    Ok(histories)
}

fn plot(idx: usize, x: &[f64], y: &[i64], predictions: &HashMap<String, Vec<f64>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut mins: Vec<f64> = y.iter().map(|&i| i as f64).collect();
    let mut maxs: Vec<f64> = y.iter().map(|&i| i as f64).collect();
    for (_, ns) in predictions.clone() {
        mins.append(&mut ns.clone());
        maxs.append(&mut ns.clone());
    }
    let min = mins.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = maxs.iter().fold(0.0f64, |a, &b| a.max(b));

    let x_range = 0 as f64 .. y.len() as f64; // must be range
    let y_range = min as f64 .. max as f64;

    let filename = format!("graphs/{idx}.png");
    let root = BitMapBackend::new(&filename, (1440, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        // .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(50)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range, y_range)?;
    chart.configure_mesh().draw()?;

    let colors: Vec<&RGBColor> = vec![&BLUE, &YELLOW, &GREEN, &CYAN, &MAGENTA, &BLACK];

    // draw original
    let graph: Vec<f64> = y.iter().map(|n| *n as f64).collect();
    let with_enumerate: Vec<(f64, f64)> = graph.iter().enumerate()
        .map(|(i, n)| (i as f64, *n as f64))
        .collect();
    // chart.draw_series(with_enumerate.clone().iter().map(|p| Circle::new(p, 4, &RED)))?
    //     .label("original")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], (&RED).clone()));
    chart.draw_series(with_enumerate.iter().map(|&p| Circle::new(p, 10, (&RED).filled())))?;
    
    // draw predictions (e.g. polynom regressor, lagrange polynoms, etc..)
    for (idx, (identifier, predictor)) in predictions.iter().enumerate() {
        let color = colors[idx%colors.iter().len()];
        let (indices, _) = get_linspace(x, N);

        // TODO: does not make sense, 
        // SSE is computed between graph (e.g.: 20 points) and predictor (e.g.: linspaced 10000) points
        // fix that

        // let sse = get_sse(&graph, &predictor);
        chart.draw_series(LineSeries::new(indices.into_iter().zip(predictor.clone()), color.clone().stroke_width(2)))?
            .label(format!("{identifier}")) // | SSE = {:.6}",  sse
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

fn get_sse(x: &[f64], y: &[f64]) -> f64 {
    x.iter().zip(y).map(|(a, b)| (a - b).abs().powi(2)).sum()
}

// POLYNOM_MAX is the maximum degree of polynom that we'll try
const POLYNOM_MAX: usize = 12;
fn get_polynomial_regressor(x: &[f64], y: &[f64]) -> (usize, Vec<f64>) {
    let mut predicted: Vec<Vec<f64>> = Vec::new();
    let mut best: (usize, f64) = (0, f64::MAX);

    for degree in 0..=POLYNOM_MAX { // 0th degree is always a line, but hey..
        let mut regressor = PolynomialRegressor::new(degree as usize);
        regressor.fit(&x, &y);

        let _y = regressor.predict(&x);
        predicted.push(_y.clone());

        let sse: f64 = get_sse(y, &_y);
        // println!("{degree}th: sse = {sse}");
        if sse < best.1 {
            best = (degree, sse);
        }
    }

    // use best to predict N points
    let mut regressor = PolynomialRegressor::new(best.0);
    regressor.fit(&x, &y);

    // interpolate/predict over range
    let (_, range) = get_linspace(x, N);
    (best.0, regressor.predict(&range))
}

fn get_linspace(x: &[f64], n: i32) -> (Vec<f64>, Vec<f64>) {
    let min = x.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = x.iter().fold(0.0f64, |a, &b| a.max(b));

    let step: f64 = (max - min) / (n as f64);
    let mut range: Vec<f64> = (0..n).map(|n| min + ((n as f64) * step)).collect();
    range.push(max);
    
    let indices: Vec<f64> = range.iter().enumerate().map(|(i, _)| i as f64 * step).collect();
    (indices, range)
}

// https://en.wikipedia.org/wiki/Lagrange_polynomial
// calculate lagrange polynomial and automatically predict for X
fn get_lagrange_interpol_polynom(x: &[f64], y: &[f64]) -> Vec<f64> {
    let (_, range) = get_linspace(x, N);
    
    // Note: I won't use the second barycentric form --> I simply don't get it
    // use_original_definition(range, x, y)
    use_first_barycentric_form(range, x, y) // more computationally friendly
}

// https://en.wikipedia.org/wiki/Lagrange_polynomial#Barycentric_form
// comprises three parts
fn use_first_barycentric_form(range: Vec<f64>, x: &[f64], y: &[f64]) -> Vec<f64> {
    // 2. barycentric weights, node specific (can be computed once)
    let w: Vec<f64> = (0..x.iter().len()).map(|j| {
        (0..x.iter().len()).filter(|&m| j != m)
            .map(|m| 1.0 / (x[j] - x[m]))
            .fold(1.0, |acc, x| acc * x)
    }).collect();

    range.iter().map(move |&p| -> f64 {
        // precache for p == nodes_j as it will end in 0 otherwise
        if let Some(idx) = x.iter().position(|&xx| xx == p) {
            return y[idx]
        }

        // 1. base, common to every polynomial
        let l: f64 = (0..x.iter().len()).map(|m| (p - x[m])).fold(1.0, |acc, x| acc * x);

        // 3. displacement
        let displacement: Vec<f64> = (0..x.iter().len()).map(|j| p - x[j]).collect();
        assert_eq!(w.iter().len(), displacement.iter().len());

        // putting all together
        l * (0..x.iter().len()).map(|j| (w[j] / displacement[j]) * y[j]).sum::<f64>()
    }).collect()
}

fn use_original_definition(range: Vec<f64>, x: &[f64], y: &[f64]) -> Vec<f64> {
    range.iter().map(move |&p| {
        let bases: Vec<f64> = _get_lagrange_bases(p, x);
        bases.iter().zip(y).map(|by| by.0 * by.1).sum() // intergrating polynomial (l0(x)*y0 + .. + ln(x)*yn)
    }).collect()
}

fn _get_lagrange_bases(p: f64, x: &[f64]) -> Vec<f64> {
    (0..x.iter().len()).map(|i| { // intergration polynom, l0, l1, ... ln
        (0..x.iter().len()).filter(|&j| i != j) // evaluate per li
            .map(|j| (p - x[j]) / (x[i] - x[j])) 
            .fold(1.0, |acc, x| acc * x)
    }).collect()
}