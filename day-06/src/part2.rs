use nom::number;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let lines =  _input.lines().map(|line| { line.chars().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>();
    let h = lines.len() - 1;

    let mut result = 0;
    let mut operator = ' ';
    let mut numbers = vec![];
    for (n ,c) in lines[4].iter().enumerate() {
        if lines[4][n] != ' ' {
            operator = lines[4][n];
            println!("operator is {}", operator);
        }

        if lines[0][n] == ' ' &&  lines[1][n] == ' ' && lines[2][n] == ' ' && lines[3][n] == ' ' && lines[4][n] == ' ' {
            result = result + compute(operator, &numbers);
            println!("intermediate result is {}", result);
            numbers.clear();
            continue;
        }

        println!("chars are: {} {} {} {}", lines[0][n], lines[1][n], lines[2][n], lines[3][n]);
        let num = assemble_number(lines[0][n], lines[1][n], lines[2][n], lines[3][n]);
        numbers.push(num);
    }


    Ok(result.to_string())
}

fn compute(operator: char, numbers: &Vec<u64>) -> u64 {
    match operator {
        '*' => { 
            let mut sum = 1;
            println!("output: operator is {}, nunbers {:?}", operator, numbers);
            for n in numbers {
                sum = sum * n;
            }
            sum
        }
        '+' => {
            let mut sum = 0;
            println!("output: operator is {}, nunbers {:?}", operator, numbers);
            for n in numbers {
                sum = sum + n;
            }
            sum
        }
        _ => {
            println!("no output: operator is {}, nunbers {:?}", operator, numbers);
            0
        }
    }
}

fn assemble_number(ch1: char, ch2: char, ch3: char, ch4: char) -> u64 {
    let (mut n1, mut n2, mut n3, mut n4) = (0, 0, 0, 0);
    if ch1 != ' ' { n1 = ch1.to_string().parse::<u64>().unwrap() }
    if ch2 != ' ' { n2 = ch2.to_string().parse::<u64>().unwrap() }
    if ch3 != ' ' { n3 = ch3.to_string().parse::<u64>().unwrap() }
    if ch4 != ' ' { n4 = ch4.to_string().parse::<u64>().unwrap() }
    if n4 != 0 {
        return n1 * 1000 + n2 * 100 + n3 * 10 + n4
    } else if n3 != 0 {
        return n1 * 100 + n2 * 10 + n3
    } else if n2 != 0 {
        return n1 * 10 + n2
    } else {
        return n1
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
