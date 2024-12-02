use std::env;
use std::path::Path;
use std::fs::read_to_string;
use plotters::prelude::*;

mod predictors;
use predictors::{get_linspace, get_sse, LagrangeInterpolator, PolynomialPredictor, Predictor};

// N is the number of points for which we visualize the predictors
const N: i32 = 100_000;

// TODO: use rayon iter for everything
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

        // dynamic types
        // https://doc.rust-lang.org/book/ch17-02-trait-objects.html
        let mut predictors: Vec<Box<dyn Predictor>> = Vec::new();
        predictors.push(Box::new(PolynomialPredictor::new(&x, &y)));
        predictors.push(Box::new(LagrangeInterpolator::new(&x, &y)));
        for predictor in predictors.iter_mut() { predictor.fit() }

        // plot graph with predictions
        if let Err(e) = plot(idx, &x, history, &predictors) {
            panic!("nooo {e}");
        }

        for predictor in predictors {
            let _y = predictor.predict_range(&x);
            println!("SSE for {}: {}", predictor.name(), get_sse(&y, &_y))
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

fn plot(idx: usize, x: &[f64], y: &[i64], predictors: &Vec<Box<dyn Predictor>>) -> Result<(), Box<dyn std::error::Error>> {
    let (_, range) = get_linspace(x, N);

    let mut mins: Vec<f64> = y.iter().map(|&i| i as f64).collect();
    let mut maxs: Vec<f64> = y.iter().map(|&i| i as f64).collect();
    for predictor in predictors {
        let predicted = predictor.predict_range(&range);
        mins.append(&mut predicted.clone());
        maxs.append(&mut predicted.clone());
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
    
    // draw predictions
    let mut idx = 0;
    for predictor in predictors {
        let stroke_width = predictors.len() - idx + 2;
        let color = colors[idx%colors.iter().len()];
        let (indices, values) = get_linspace(x, N);
        let predicted = predictor.predict_range(&values);

        chart.draw_series(LineSeries::new(
            indices.into_iter().zip(predicted), 
            color.clone().stroke_width(stroke_width as u32)))?
            .label(predictor.name()) // | SSE = {:.6}",  sse
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
        idx = idx + 1;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}