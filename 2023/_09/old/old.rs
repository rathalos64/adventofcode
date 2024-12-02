use std::env;
use std::path::Path;
use std::fs::read_to_string;
use plotters::prelude::*;
use compute::predict::PolynomialRegressor;

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

    // // plotting
    // for (i, history) in histories.iter().enumerate() {
    //     println!("{i}| draw graph ({:?})", history);
    //     if let Err(e) = plot_numbers(i as u32, history) {
    //         panic!("{e}")
    //     }
    // }

    // using polynomial regression
    for (i, history) in histories.iter().enumerate() {
        let _x: Vec<f64> = (0 .. history.len()).map(|n| n as f64).collect();
        let _y: Vec<f64> = history.iter().map(|n| *n as f64).collect();

        println!("{i}| predict with ({:?})", history);
        let mut diffs: Vec<f64> = Vec::new();
        let mut predicted: Vec<Vec<f64>> = Vec::new();
        for j in 0..10 {
            let mut regressor = PolynomialRegressor::new(j);
            regressor.fit(&_x, &_y);
            // println!("{j}th pol: coeffs -> {:?}", regressor.coef);
            
            let y = regressor.predict(&_x);
            // println!("predicted x = {:?} with y = {:?}", _x, y);
            predicted.push(y.clone());

            // calculate differences
            let diff: Vec<f64> = _y.clone().into_iter().zip(y).map(|(a, b)| (a - b).abs().powi(2)).collect();
            let sum_of_squared = diff.iter().sum::<f64>();
            diffs.push(sum_of_squared);
            println!("{j}th pol: sum of squared diff {}", sum_of_squared);
        }
        let index_of_min: i32 = diffs
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index as i32).unwrap() + 1;
        println!("best polynomial degree: {index_of_min}");

        if let Err(e) = plot_numbers(i as u32, history, &predicted) {
            panic!("nooo {e}");
        }
        println!();
        break;
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

fn plot_numbers(i: u32, graph: &Vec<i64>, predicted_polynomials: &Vec<Vec<f64>>) -> Result<(), Box<dyn std::error::Error>> {
    let x_range = 0 as f32 .. graph.len() as f32;
    let y_range = (graph.clone().into_iter().min()).unwrap() as f32
                .. (graph.clone().into_iter().max()).unwrap() as f32;

    let filename = format!("graphs/{}.png", i);
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
    let with_enumerate: Vec<(f32, f32)> = graph.clone().iter().enumerate()
        .map(|(i, n)| (i as f32, *n as f32))
        .collect();
    
    chart.draw_series(LineSeries::new(with_enumerate, (&RED).stroke_width(10)))?
        .label("original")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], (&RED).clone()));
    
    for (p, polynomial) in predicted_polynomials.iter().enumerate() {
        let color = colors[p%colors.iter().len()];
        let mapped: Vec<(f32, f32)> = polynomial.clone().iter().enumerate()
            .map(|(i, n)| (i as f32, *n as f32))
            .collect();
        chart
            .draw_series(LineSeries::new(mapped, color.clone().stroke_width(2)))?
            .label(format!("p^{}", p+1))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
    }

    // .label("y = x^2")
    // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
