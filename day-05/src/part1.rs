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
    for rg in &ranges {
        for n in &mut input {
            if *n == 0 { continue }
            else {
                if rg.0 <= *n && rg.1 >= *n {
                    fresh_count = fresh_count + 1;
                    *n = 0;
                }
            }
        }
    }

    Ok(fresh_count.to_string())
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
