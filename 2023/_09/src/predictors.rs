use compute::predict::PolynomialRegressor;

// should be references to not consume the underlying Box<> reference
// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
// https://stackoverflow.com/a/76038535 
pub trait Predictor {
    fn name(&self) -> String;
    fn fit(&mut self);
    fn predict(&self, x: f64) -> f64;
    fn predict_range(&self, x: &[f64]) -> Vec<f64>;
}

pub fn get_sse(x: &[f64], y: &[f64]) -> f64 {
    x.iter().zip(y).map(|(a, b)| (a - b).abs().powi(2)).sum()
}

pub fn get_linspace(x: &[f64], n: i32) -> (Vec<f64>, Vec<f64>) {
    let min = x.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = x.iter().fold(0.0f64, |a, &b| a.max(b));

    let step: f64 = (max - min) / (n as f64);
    let mut range: Vec<f64> = (0..n).map(|n| min + ((n as f64) * step)).collect();
    range.push(max);
    
    let indices: Vec<f64> = range.iter().enumerate().map(|(i, _)| i as f64 * step).collect();
    (indices, range)
}

pub struct PolynomialPredictor {
    x: Vec<f64>,
    y: Vec<f64>,

    regressor: PolynomialRegressor,
    degree: usize
}
impl PolynomialPredictor {
    const POLYNOM_MAX: usize = 12;

    pub fn new(x: &[f64], y: &[f64]) -> Self {
        PolynomialPredictor{
            x: x.to_vec(),
            y: y.to_vec(),
            regressor: PolynomialRegressor::new(0),
            degree: 0
        }
    }
}
impl Predictor for PolynomialPredictor {
    fn name(&self) -> String {
        format!("poly_reg {}th deg", self.degree)
    }

    fn fit(&mut self) {
        let mut best: (usize, f64) = (0, f64::MAX);

        for degree in 0..=Self::POLYNOM_MAX { // 0th degree is always a line, but hey..
            let mut regressor = PolynomialRegressor::new(degree as usize);
            regressor.fit(&self.x, &self.y);

            let _y = regressor.predict(&self.x);
            let sse: f64 = get_sse(&self.y, &_y);
            if sse < best.1 {
                best = (degree, sse);
            }
        }
        self.regressor = PolynomialRegressor::new(best.0);
        self.regressor.fit(&self.x, &self.y);
    }

    fn predict(&self, x: f64) -> f64 {
        self.regressor.predict(&vec![x])[0]
    }

    fn predict_range(&self, x: &[f64]) -> Vec<f64> {
        self.regressor.predict(x)
    }
}

#[derive(PartialEq, Clone)]
enum LagrangeMode {
    Original, // original is computationally unfriendly
    FirstBarycentricForm
    // second barycentric form is too complicated for me
}

// https://en.wikipedia.org/wiki/Lagrange_polynomial
#[derive(Clone)]
pub struct LagrangeInterpolator {
    x: Vec<f64>,
    y: Vec<f64>,
    mode: LagrangeMode,

    w: Vec<f64> // barycentric weights
}
impl LagrangeInterpolator {
    const LAGRANGE_FORM: LagrangeMode = LagrangeMode::FirstBarycentricForm;

    pub fn new(x: &[f64], y: &[f64]) -> Self {
        LagrangeInterpolator{
            x: x.to_vec(), 
            y: y.to_vec(),
            mode: LagrangeInterpolator::LAGRANGE_FORM,
            w: Vec::new(),
        }
    }
}
impl Predictor for LagrangeInterpolator {
    fn name(&self) -> String {
        String::from("lagrange interpol poly")
    }

    fn fit(&mut self) {
        if self.mode == LagrangeMode::FirstBarycentricForm {
            // weights can be computed once
            self.w = (0..self.x.iter().len()).map(|j| {
                (0..self.x.iter().len()).filter(|&m| j != m)
                    .map(|m| 1.0 / (self.x[j] - self.x[m]))
                    .fold(1.0, |acc, x| acc * x)
            }).collect()
        }
    }

    fn predict(&self, x: f64) -> f64 {
        let indices = 0..self.x.iter().len();
        
        match self.mode {
            LagrangeMode::Original => {
                // intergration polynom, l0, l1, ... ln
                let bases: Vec<f64> = indices.clone().map(|i| { 
                    indices.clone().filter(|&j| i != j) // evaluate per li
                        .map(|j| (x - self.x[j]) / (self.x[i] - self.x[j])) 
                        .fold(1.0, |acc, x| acc * x)
                }).collect();

                bases.iter().zip(self.y.iter()).map(|by| by.0 * by.1).sum()
            }

            LagrangeMode::FirstBarycentricForm => {
                // https://en.wikipedia.org/wiki/Lagrange_polynomial#Barycentric_form
                // precache for p == nodes_j as it will end in 0 otherwise
                if let Some(idx) = self.x.iter().position(|&xx| xx == x) {
                    return self.y[idx]
                }

                // base, common to every polynomial
                let l: f64 = indices.clone().map(|m| (x - self.x[m])).fold(1.0, |acc, x| acc * x);

                // displacement
                let displacement: Vec<f64> = indices.clone().map(|j| x - self.x[j]).collect();
                assert_eq!(self.w.iter().len(), displacement.iter().len());

                // putting all together
                l * indices.clone().map(|j| (self.w[j] / displacement[j]) * self.y[j]).sum::<f64>()
            }
        }
    }

    fn predict_range(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|&x| self.clone().predict(x)).collect()
    }
}
