#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let lines: Vec<Vec<&str>> = _input.lines().map(|line| line.split_whitespace().collect()).collect();
    let mut result = 0;

    for (n, operator) in lines[4].iter().enumerate() {
        println!("{} {} {} {} {}", lines[0][n], lines[1][n], lines[2][n], lines[3][n], lines[4][n]);
        let temp = match *operator {
            "*" => lines[0][n].parse::<u64>().unwrap() * lines[1][n].parse::<u64>().unwrap() * lines[2][n].parse::<u64>().unwrap() * lines[3][n].parse::<u64>().unwrap() ,
            "+" => lines[0][n].parse::<u64>().unwrap() + lines[1][n].parse::<u64>().unwrap() + lines[2][n].parse::<u64>().unwrap() + lines[3][n].parse::<u64>().unwrap() ,
            _ => 0
        };

        result = result + temp;
    }


    Ok(result.to_string())
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
