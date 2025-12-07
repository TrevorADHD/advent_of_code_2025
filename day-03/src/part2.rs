#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    let mut sum = 0;

    for line in _input.lines() {
        sum  = sum + get_max_online(line);
    }

    Ok(sum.to_string())
}

fn get_max_online( line: &str) -> u64{

    
    let digits = string_to_vec_digtis(line);
    
    let mut result = vec![];
    let len = 12;
    let l = digits.len() - 1;
    let mut start = 0;

    for n in (0..12).rev() {
        let end = l - n;
        start = find_max_pos(start, end, &digits) + 1;
        result.push(digits[start - 1]);
    }
    
    get_result(&result).parse::<u64>().unwrap()

}

fn get_result(result: &Vec<u32>) -> String {
    let mut r = "".to_string();
    for d in result {
        r = r + &d.to_string();
    }

    r
}

fn find_max_pos(start: usize, end: usize, digits: &Vec<u32>) -> usize {
    let mut init_pos = end;
    for n in (start..end).rev() {
        if digits[n] >= digits[init_pos] {
            init_pos = n
        }
    }

    init_pos
}

fn string_to_vec_digtis(input: &str) -> Vec<u32>{
    input.chars().filter_map(|c| {c.to_digit(10)}).collect()
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
