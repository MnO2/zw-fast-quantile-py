use zw_fast_quantile::UnboundEpsilonSummary;
use pyo3::prelude::*;

macro_rules! generate_struct_def {
    ($name: ident, $type: ident) => {
        #[pyclass(subclass)]
        struct $name {
            summary: UnboundEpsilonSummary<$type>
        }

        #[pymethods]
        impl $name {
            #[new]
            fn new(epsilon: f64) -> Self {
                Self {
                    summary: UnboundEpsilonSummary::<$type>::new(epsilon)
                }
            }

            #[pyo3(text_signature = "($self, val)")]
            fn update(&mut self, py: Python, val: $type) {
                py.allow_threads(move || self.summary.update(val))
            }

            #[pyo3(text_signature = "($self, rank)")]
            fn query(&mut self, py: Python, rank: f64) -> $type {
                py.allow_threads(move || self.summary.query(rank))
            }
        }
    };
}

generate_struct_def!(IntQuantileSummary, i64);

#[pyclass(subclass)]
struct FloatQuantileSummary {
    summary: UnboundEpsilonSummary<ordered_float::NotNan<f64>>
}

#[pymethods]
impl FloatQuantileSummary {
    #[new]
    fn new(epsilon: f64) -> Self {
        Self {
            summary: UnboundEpsilonSummary::<ordered_float::NotNan<f64>>::new(epsilon)
        }
    }

    #[pyo3(text_signature = "($self, val)")]
    fn update(&mut self, py: Python, val: f64) {
        let v = unsafe { ordered_float::NotNan::new_unchecked(val) };
        py.allow_threads(move || self.summary.update(v));
    }

    #[pyo3(text_signature = "($self, rank)")]
    fn query(&mut self, py: Python, rank: f64) -> f64 {
        py.allow_threads(move || self.summary.query(rank).into())
    }
}


#[pymodule]
fn zw_fast_quantile_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IntQuantileSummary>()?;
    m.add_class::<FloatQuantileSummary>()?;
    Ok(())
}
