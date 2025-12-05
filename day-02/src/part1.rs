#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let ranges: Vec<&str> = _input.trim().split(',').collect();
    let mut sum:u64 = 0;

    for range in ranges {
        // println!("range: {}", range);
        let r: Vec<&str> = range.split('-').collect();
        let (start, end) = (r[0].parse::<u64>().expect(
    &format!("Failed to parse start value '{}' at index 0", r[0])),r[1].parse::<u64>().expect(
    &format!("Failed to parse end value '{}' at index 1", r[1])));
        // println!("start: {}, end: {}", start , end);
        sum = sum + run_range(start, end);
    }
    println!("sum: {}", sum);
    Ok(sum.to_string())
}

fn run_range(start: u64, end: u64) -> u64 {
    let mut sum = 0;
    for i in start..=end {
        if check_invalid(i) {
            sum = sum + i;
        }
    }

    sum
}

fn check_invalid(num: u64) -> bool {
    let n = num.to_string().len() as u64;
    // println!("num is {num}, len is {n}");
    if n % 2 == 1 {
        return false;
    } else {
        let half_size = n / 2;
        // println!("half len is {half_size}, num is {num}, left is {}, right is {}", num / (10u64.pow(half_size as u32)), num % (10u64.pow(half_size as u32)));
        return num / (10u64.pow(half_size as u32))
            == num % (10u64.pow(half_size as u32));
    }
}

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
