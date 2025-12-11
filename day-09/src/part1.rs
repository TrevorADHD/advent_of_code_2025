use std::cmp::max;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let points = _input.lines().map(|line|{
        line.split(',').map(
            |p|{ p.parse::<u64>().unwrap()}
        ).collect::<Vec<u64>>()
    }).collect::<Vec<Vec<u64>>>();
    println!("{:?}", points);
    let mut max_area = 0;

    for n in 0..points.len() - 1 {
        for m in n + 1 .. points.len() {
            max_area = max(max_area, get_area(&points[n], &points[m]));
        }
    }

    Ok(max_area.to_string())
}

fn get_area(p1: &Vec<u64>, p2: &Vec<u64>) -> u64 {
    ((p1[0]).abs_diff(p2[0]) + 1) *  ((p1[1]).abs_diff(p2[1]) + 1)
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
