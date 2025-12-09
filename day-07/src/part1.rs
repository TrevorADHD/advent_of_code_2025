use std::{collections::HashSet, marker};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let lines = _input.lines().into_iter().enumerate().filter_map(
        |(n, line)| {
            if n % 2 == 0 {
                Some(line.chars().collect::<Vec<char>>()) 
            } else {
                None
            }
            }
    ).collect::<Vec<Vec<char>>>();

    use std::collections::HashSet;
    let pos_s = find_s(&lines[0]);
    let mut marker_set = HashSet::new();
    marker_set.insert(pos_s);

    let mut result = 0;

    for n in 1..lines.len() {
        result = result + check_and_update_mask(&mut marker_set, &lines[n]);
    }

    Ok(result.to_string())
}

fn check_and_update_mask(set: &mut HashSet<usize>, line: &Vec<char>) -> i32 {
    let mut temp_set = HashSet::new();
    let mut hit_set = HashSet::new();
    let mut remove_set = HashSet::new();
    // println!("current line: {:?}", line);
    // println!("current set: {:?}", set);

    for p in set.iter() {
        if line[*p] == '^' {
            hit_set.insert(*p);
            remove_set.insert(*p);
            temp_set.insert(p - 1);
            temp_set.insert(p + 1);
        }
    }
    set.retain(|p| !remove_set.contains(p));
    set.extend(temp_set);

    hit_set.len() as i32
}

fn find_s(line: &Vec<char>) -> usize{
    for (n, c) in line.iter().enumerate() {
        if *c == 'S' {
            return n
        }
    }

    return 0
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
