#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    let mut sum = 0;

    for line in _input.lines() {
        sum  = sum + get_max_online(line);
    }

    Ok(sum.to_string())


}

fn get_max_online( line: &str) -> u32{
    let digits = string_to_vec_digtis(line);
    let l = digits.len() - 1;
    let (mut pos_h, mut pos_l) = (l - 1, l);
    let mut n  = pos_l - 1;
    while n > 0 {
        if digits[n - 1] >= digits[pos_h] {
            let temp_h = pos_h;
            pos_h = n - 1;

            let max_pos = get_max_posotion(temp_h, n, &digits);

            if digits[max_pos] >= digits[pos_l]{
                pos_l = max_pos
            }
        }

        n = n - 1;
    }

   digits[pos_h] * 10 + digits[pos_l]
}

fn string_to_vec_digtis(input: &str) -> Vec<u32>{
    input.chars().filter_map(|c| {c.to_digit(10)}).collect()
}

fn get_max_posotion(x: usize, y:usize, digits: &Vec<u32>) -> usize{
    if digits[x] >= digits[y] {
        return x
    } else {
        return y
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
