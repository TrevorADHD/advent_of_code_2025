

use regex::Regex;


#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut zero_count = 0;
    let mut sum: i32 = 50;
    let mut n = 0 ;
    for rotation in _input.lines() {
        // if n >= 20 { break }
        let re = Regex::new(r"(R|L)(\d+)").unwrap();
        // print!("rotation is {}", rotation);
        if let Some(caps) = re.captures(rotation) {
            println!("{}, {}", &caps[1], &caps[2]);
            let derection = &caps[1];
            let steps = &caps[2].parse::<i32>().unwrap();
            match derection {
                "L" => {
                    (sum, zero_count) = adjust_sum(sum - *steps as i32, zero_count)
                }

                "R" => {
                    (sum, zero_count) = adjust_sum(sum + *steps as i32, zero_count)
                }
                _ => {}
            }
        }

        println!("sum is {}, zero is {}", sum, zero_count);
        n = n + 1;
    }
    
    Ok(zero_count.to_string())
}

fn adjust_sum(mut sum: i32,mut zero: i32) -> (i32, i32) {
    sum = sum % 100;
    if sum == 0 {
        zero = zero + 1;
    }

    (sum, zero)
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
