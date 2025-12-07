use core::hash;
use std::{cmp::max,cmp::min, collections::{HashMap, HashSet}};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut ranges = vec![];
    let mut input = vec![];
    let lines: Vec<&str> = _input.lines().collect();

    let mut parsing_ranges = true;
    for line in lines {
        if line == "" {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            ranges.push(range_processor(line));
        } else {
            input.push(input_processor(line));
        }
    }

    let mut fresh_count = 0;
    // let mut ranges_arr: Vec<(u64, u64)> = vec![];
    let mut ranges_set: HashSet<(u64, u64)> = HashSet::new();
    for rg in &ranges {
        traversal(*rg, &mut ranges_set);
    }

    for n in ranges_set {
       fresh_count = fresh_count + (n.1 - n.0 + 1);
    }

    Ok(fresh_count.to_string())
}



fn traversal(input: (u64, u64), range_set: &mut HashSet<(u64, u64)>) {
    let to_remove: Vec<(u64, u64)> = range_set.iter()
        .filter(|r| is_overlap(&input, r))
        .copied()
        .collect();
    
    let mut final_range: (u64, u64) = input;
    let has_overlaps = to_remove.len() > 0;

    for r in to_remove {
        final_range = merge_ranges(&final_range, &r);
        range_set.remove(&r);
    }

    if !has_overlaps {
        range_set.insert(input);
    } else {
        traversal(final_range, range_set);
    }
}

fn is_overlap(a: &(u64, u64), b: &(u64, u64)) -> bool{
    return a.0 <= b.1 && a.1 >= b.0 ;
}

// a is input , b is in map
fn merge_ranges(a: &(u64, u64), b: &(u64, u64)) -> (u64, u64) {
        return (min(a.0, b.0), max(a.1, b.1))
}

fn range_processor(input : &str) -> (u64, u64) {
    let parts: Vec<&str> = input.split('-').collect();
    let (x, y) = (parts[0], parts[1]);
    (x.parse().unwrap(), y.parse().unwrap())
}

fn input_processor(input : &str) -> u64{
    input.parse().unwrap()
}

// pub fn brute_force_solution(_input: &str) -> miette::Result<String> {
    
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
