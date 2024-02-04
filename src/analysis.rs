#[derive(Debug)]
pub struct Stats {
    pub n: usize,
    pub min: usize,
    pub max: usize,
    pub sum: usize,
    pub avg: f64,
    pub sd: f64,
    pub median: usize,
    pub pct90: usize,
    pub pct95: usize,
    pub pct99: usize,
}

impl Stats {
    pub fn from_iter<I>(iter: I) -> Option<Stats>
    where
        I: Iterator<Item = usize> + Clone,
    {
        let n = iter.clone().count();
        if n == 0 {
            return None;
        }

        let min = iter.clone().min()?;
        let max = iter.clone().max()?;
        let sum: usize = iter.clone().sum();
        let avg = sum as f64 / n as f64;
        let sd = (iter
            .clone()
            .map(|v| v as f64)
            .fold(0f64, |acc, v| acc + (v - avg).powf(2f64))
            / n as f64)
            .sqrt();

        let median = if n % 2 == 1 {
            iter.clone().nth((n + 1) / 2 - 1)?
        } else {
            let l = iter.clone().nth(n / 2 - 1)?;
            let r = iter.clone().nth(n / 2)?;
            (l + r) / 2usize
        };

        let pct90 = get_nth_percentile(n, iter.clone(), 0.90)?;
        let pct95 = get_nth_percentile(n, iter.clone(), 0.95)?;
        let pct99 = get_nth_percentile(n, iter.clone(), 0.99)?;

        Some(Stats {
            n,
            min,
            max,
            sum,
            avg,
            sd,
            median,
            pct90,
            pct95,
            pct99,
        })
    }
}

fn get_nth_percentile<I>(count: usize, times: I, percentile: f64) -> Option<usize>
where
    I: Iterator<Item = usize> + Clone,
{
    let el = count as f64 * percentile;
    let el_trunc = el as isize - 1;
    if el_trunc < 0 {
        return times.clone().next();
    }

    if el_trunc as usize + 1 >= count {
        return times.clone().nth(el_trunc as usize);
    }

    let el_a = times.clone().nth(el_trunc as usize)?;
    let el_b = times.clone().nth(el_trunc as usize + 1)?;

    let el_fract_b = el - el_trunc as f64;
    let el_fract_a = 1f64 - el_fract_b;

    let res = (el_a as f64 * el_fract_a + el_b as f64 * el_fract_b).round();

    Some(res as usize)
}
