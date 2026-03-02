use pyo3::prelude::*;
use pythonize::pythonize;
use serde::Serialize;
use ::zxcvbn::time_estimates::CrackTimeSeconds;
use ::zxcvbn::{zxcvbn as zxcvbn_core, Match, feedback::Warning};

#[derive(Serialize)]
struct CrackTimesSecondsOut {
    online_throttling_100_per_hour: f64,
    online_no_throttling_10_per_second: f64,
    offline_slow_hashing_1e4_per_second: f64,
    offline_fast_hashing_1e10_per_second: f64,
}

#[derive(Serialize)]
struct CrackTimesDisplayOut {
    online_throttling_100_per_hour: String,
    online_no_throttling_10_per_second: String,
    offline_slow_hashing_1e4_per_second: String,
    offline_fast_hashing_1e10_per_second: String,
}

#[derive(Serialize)]
struct FeedbackOut {
    warning: Option<String>,
    suggestions: Vec<String>,
}

#[derive(Serialize)]
struct ZxcvbnResultOut {
    guesses: u64,
    guesses_log10: f64,
    score: u8,
    feedback: FeedbackOut,
    sequence: Vec<Match>,
    calc_time: f64,
    crack_times_seconds: CrackTimesSecondsOut,
    crack_times_display: CrackTimesDisplayOut,
}

fn crack_time_to_f64(value: CrackTimeSeconds) -> f64 {
    match value {
        CrackTimeSeconds::Integer(v) => v as f64,
        CrackTimeSeconds::Float(v) => v,
    }
}

#[pyfunction(name = "zxcvbn", signature = (password, user_inputs=None))]
fn zxcvbn_py(
    py: Python<'_>,
    password: &str,
    user_inputs: Option<Vec<String>>,
) -> PyResult<Py<PyAny>> {
    let inputs = user_inputs.unwrap_or_default();
    let input_refs = inputs.iter().map(String::as_str).collect::<Vec<_>>();
    let entropy = zxcvbn_core(password, &input_refs);
    let crack_times = entropy.crack_times();
    let t_online_throttling = crack_times.online_throttling_100_per_hour();
    let t_online_no_throttling = crack_times.online_no_throttling_10_per_second();
    let t_offline_slow = crack_times.offline_slow_hashing_1e4_per_second();
    let t_offline_fast = crack_times.offline_fast_hashing_1e10_per_second();

    let feedback = if let Some(feedback) = entropy.feedback() {
        FeedbackOut {
            warning: feedback.warning().map(|value: Warning| value.to_string()),
            suggestions: feedback
                .suggestions()
                .iter()
                .map(ToString::to_string)
                .collect(),
        }
    } else {
        FeedbackOut {
            warning: None,
            suggestions: Vec::new(),
        }
    };

    let response = ZxcvbnResultOut {
        guesses: entropy.guesses(),
        guesses_log10: entropy.guesses_log10(),
        score: u8::from(entropy.score()),
        feedback,
        sequence: entropy.sequence().to_vec(),
        calc_time: entropy.calculation_time().as_secs_f64() * 1000.0,
        crack_times_seconds: CrackTimesSecondsOut {
            online_throttling_100_per_hour: crack_time_to_f64(t_online_throttling),
            online_no_throttling_10_per_second: crack_time_to_f64(t_online_no_throttling),
            offline_slow_hashing_1e4_per_second: crack_time_to_f64(t_offline_slow),
            offline_fast_hashing_1e10_per_second: crack_time_to_f64(t_offline_fast),
        },
        crack_times_display: CrackTimesDisplayOut {
            online_throttling_100_per_hour: t_online_throttling.to_string(),
            online_no_throttling_10_per_second: t_online_no_throttling.to_string(),
            offline_slow_hashing_1e4_per_second: t_offline_slow.to_string(),
            offline_fast_hashing_1e10_per_second: t_offline_fast.to_string(),
        },
    };

    let py_obj = pythonize(py, &response)?;

    Ok(py_obj)
}

#[pymodule]
fn _zxcvbn_rs(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(zxcvbn_py, module)?)?;
    Ok(())
}
