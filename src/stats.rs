use crate::model::LatencySummary;

#[derive(Debug, Default, Clone)]
pub struct OnlineStats {
    n: u64,
    mean: f64,
    m2: f64,
}

impl OnlineStats {
    pub fn push(&mut self, x: f64) {
        self.n += 1;
        let delta = x - self.mean;
        self.mean += delta / (self.n as f64);
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }

    pub fn stddev(&self) -> Option<f64> {
        if self.n < 2 {
            None
        } else {
            Some((self.m2 / ((self.n - 1) as f64)).sqrt())
        }
    }
}

pub fn latency_summary_from_samples(
    sent: u64,
    received: u64,
    samples_ms: &[f64],
    jitter_ms: Option<f64>,
) -> LatencySummary {
    let loss = if sent == 0 {
        0.0
    } else {
        ((sent - received) as f64) / (sent as f64)
    };

    if samples_ms.is_empty() {
        return LatencySummary {
            sent,
            received,
            loss,
            min_ms: None,
            mean_ms: None,
            median_ms: None,
            p25_ms: None,
            p75_ms: None,
            max_ms: None,
            jitter_ms,
        };
    }

    // Use the same calculation method as metrics.rs for consistency
    let mut sorted = samples_ms.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();

    let min_ms = Some(sorted[0]);
    let max_ms = Some(sorted[n - 1]);

    // Compute metrics using the same method as metrics.rs
    if let Some((mean, median, p25, p75)) = crate::metrics::compute_metrics(samples_ms.to_vec()) {
        // Compute jitter (stddev) if not provided
        let jitter = jitter_ms.unwrap_or_else(|| {
            let variance = samples_ms.iter().map(|&x| (x - mean).powi(2)).sum::<f64>()
                / samples_ms.len() as f64;
            variance.sqrt()
        });

        LatencySummary {
            sent,
            received,
            loss,
            min_ms,
            mean_ms: Some(mean),
            median_ms: Some(median),
            p25_ms: Some(p25),
            p75_ms: Some(p75),
            max_ms,
            jitter_ms: Some(jitter),
        }
    } else {
        LatencySummary {
            sent,
            received,
            loss,
            min_ms: None,
            mean_ms: None,
            median_ms: None,
            p25_ms: None,
            p75_ms: None,
            max_ms: None,
            jitter_ms,
        }
    }
}
