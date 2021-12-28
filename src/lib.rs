use zw_fast_quantile::UnboundEpsilonSummary;
use pyo3::prelude::*;

#[pyclass(subclass)]
struct QuantileSummary {
    summary: UnboundEpsilonSummary<isize>
}

#[pymethods]
impl QuantileSummary {
    #[new]
    fn new(epsilon: f64) -> Self {
        Self {
            summary: UnboundEpsilonSummary::<isize>::new(epsilon)
        }
    }

    #[pyo3(text_signature = "($self, val)")]
    fn update(&mut self, py: Python, val: isize) {
        py.allow_threads(move || self.summary.update(val))
    }

    #[pyo3(text_signature = "($self, rank)")]
    fn query(&mut self, py: Python, rank: f64) -> isize {
        py.allow_threads(move || self.summary.query(rank))
    }
}

#[pymodule]
fn zw_fast_quantile_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QuantileSummary>()?;
    Ok(())
}
