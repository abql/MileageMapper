use num_format::{Locale, ToFormattedString};
use rand::{Rng, thread_rng};
use mileage_mapper::{EntityType, parse_args};

fn linear(input: &mut i64, amount: i16, step: i64) -> Vec<i64> {
    let mut linear_vals: Vec<i64> = Vec::new();

    for _ in 0..amount {
        linear_vals.push(*input);
        *input += step;
        linear_vals.push(*input);
    }

    return linear_vals
}

fn oscillating(input: &mut i64, step: i64, lo_bound: i8, hi_bound: i8, pattern: Vec<i16>) -> Vec<i64> {
    let mut segments: Vec<i16> = Vec::new();
    let mut oscil_vals: Vec<i64> = Vec::new();

    for v in pattern {
        segments.push(v)
    }

    for s in segments {
        for x in linear(input, s, step) {
            oscil_vals.push(x);
        }
        let rnd_amount = thread_rng().gen_range(lo_bound..hi_bound);
        *input += i64::from(rnd_amount);
    }

    return oscil_vals
}

fn en_format(target: i64) -> String {
    return target.to_formatted_string(&Locale::en)
}

fn main() {

    let args = parse_args();

    match args.entity_type {
        EntityType::Osc(mut osc_values) => {
            for pair in oscillating(&mut osc_values.start_amount, osc_values.increment, osc_values.lo_offset, osc_values.hi_offset, osc_values.pattern).chunks_exact(2) {
                println!("{} - {}", en_format(pair[0]), en_format(pair[1]));
            }
        }
        EntityType::Lin(mut linear_amount) => {
            for pair in linear(&mut linear_amount.start_amount, linear_amount.amount, linear_amount.increment).chunks_exact(2) {
                println!("{} - {}", en_format(pair[0]), en_format(pair[1]));
            }
        }
    }
}
