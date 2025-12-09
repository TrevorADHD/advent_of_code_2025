use std::{collections::HashSet, result};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {

    let points = _input.lines().map(|line| {
        line.split(',').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()
    }).collect::<Vec<Vec<u64>>>();

    let mut dist_arr : Vec<Distance> = vec![];
    for a in 0..points.len() - 1 {
        for b in a+1..points.len() {
            let dist = compute_distance(&points[a], &points[b]);
            dist_arr.push(Distance { p1: a, p2: b, dist: dist });
        }
    }
    dist_arr.sort_by(|a, b| { a.dist.cmp(&b.dist)});

    // println!("{:?}" , dist_arr);
    // println!("{:?}" , dist_arr.len());

    let mut result_pool: Vec<HashSet<usize>> = vec![];
    for n in 0..points.len() {
        let mut new_rp = HashSet::new();
        new_rp.insert(n);
        result_pool.push(new_rp);
    }
    
    for n in dist_arr[0..1000].iter() {
        let idx1 = result_pool.iter().position(|rp| rp.contains(&n.p1)).unwrap();
        let idx2 = result_pool.iter().position(|rp| rp.contains(&n.p2)).unwrap();
        if idx1 != idx2 {
            let rp2_clone = result_pool[idx2].clone();
            result_pool[idx1].extend(rp2_clone.iter());
            result_pool.remove(idx2);
        }
    }

    let mut result = 0;
    for n in dist_arr[1000..dist_arr.len()].iter() {
        let idx1 = result_pool.iter().position(|rp| rp.contains(&n.p1)).unwrap();
        let idx2 = result_pool.iter().position(|rp| rp.contains(&n.p2)).unwrap();
        if idx1 != idx2 {
            let rp2_clone = result_pool[idx2].clone();
            result_pool[idx1].extend(rp2_clone.iter());
            result_pool.remove(idx2);
        }

        if result_pool.len() == 1 {
            result = points[n.p1][0] * points[n.p2][0];
            break;
        }
    }

    // result_pool.sort_by(|a, b| {a.len().cmp(&b.len())} );
    // println!("{:?}", result_pool);
    // let rps_len = result_pool.len();
    // println!("{:?}", rps_len);

    // let result = result_pool[rps_len - 1].len() * result_pool[rps_len - 2].len() * result_pool[rps_len - 3].len();     
    // todo!("day 01 - part 1");
    Ok(result.to_string())
}

fn compute_distance(p1: &Vec<u64>, p2: &Vec<u64>) -> u64 {
    let dx = p1[0] as i64 - p2[0] as i64;
    let dy = p1[1] as i64 - p2[1] as i64;
    let dz = p1[2] as i64 - p2[2] as i64;
    (dx.pow(2) + dy.pow(2) + dz.pow(2)) as u64
}

#[derive(Debug)]
struct Distance {
    p1 : usize,
    p2 : usize,
    dist : u64,
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
