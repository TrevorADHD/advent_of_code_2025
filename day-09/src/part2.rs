use std::{cmp::{max, min}, collections::HashMap, hash::Hash};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let points = _input.lines().map(|line|{
        line.split(',').map(
            |p|{ p.parse::<usize>().unwrap()}
        ).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();
    println!("{:?}", points);
    let mut max_area = 0;

    let mut panel: HashMap<usize ,Vec<usize>> = HashMap::new();

    // draw the horizontal scan
    for n in 0..points.len() - 1 {
        draw_horizontal_scan(&points[n], &points[n+1], &mut panel);
    }
    draw_horizontal_scan(&points[points.len() - 1], &points[0], &mut panel);

    println!("{:?}", panel);

    // fill the inside


    // check from biggest

    Ok(max_area.to_string())
}

fn draw_horizontal_scan(p1: &Vec<usize>, p2: &Vec<usize>, panel: &mut HashMap<usize, Vec<usize>>) {
    if p1[1] == p2[1] {
        if let None = panel.get(&p1[1]) {
            panel.insert(p1[1], vec![]);
        }

        let v = panel.get_mut(&p1[1]).unwrap();
        v.push(p1[0]);
        v.push(p2[0]);
        v.sort();

    } else {
        let (h ,l) = (max(p1[1], p2[1]),min(p1[1], p2[1]));
        for n in l + 1 .. h {
            if let None = panel.get(&n) {
                panel.insert(n, vec![]);
            }

            let v = panel.get_mut(&n).unwrap();
            v.push(p1[0]);
            v.sort();
        }
    }
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
