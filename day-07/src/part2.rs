use std::{collections::{HashMap, HashSet}, marker};

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


    use std::collections::HashMap;
    let pos_s = find_s(&lines[0]);
    let mut marker_map = HashMap::new();
    marker_map.insert(pos_s, 1i64);

    let mut result = 0;
    // stream_path.push(marker_set.clone());
    for n in 1..lines.len() {
        // result = result + 
        check_and_update_mask(&mut marker_map, &lines[n]);
    }

    for n in marker_map.values() {
        result = result + n;
    }


    Ok(result.to_string())
}


fn check_and_update_mask(set: &mut HashMap<usize, i64>, line: &Vec<char>) -> i64 {
    let mut temp_set = HashMap::new();
    let mut hit_set = HashSet::new();
    let mut remove_set = HashSet::new();
    // println!("current line: {:?}", line);
    // println!("current set: {:?}", set);

    for p in set.iter() {
        if line[*p.0] == '^' {
            hit_set.insert(*p.0);
            remove_set.insert(*p.0);
            temp_set.insert(*p.0 - 1, temp_set.get(&(*p.0 - 1)).unwrap_or(&0) + set.get(p.0).unwrap_or(&0));
            temp_set.insert(*p.0 + 1, temp_set.get(&(*p.0 + 1)).unwrap_or(&0) + set.get(p.0).unwrap_or(&0));
        }
    }
    println!("temp set: {:?}", temp_set);
    set.retain(|p, _| !remove_set.contains(p));
    // set.extend(temp_set);
    merge_counts_existing_only(set, temp_set);
    println!("updated set: {:?}", set);
    
    // stream_path.push(set.clone());

    hit_set.len() as i64
}

fn find_s(line: &Vec<char>) -> usize{
    for (n, c) in line.iter().enumerate() {
        if *c == 'S' {
            return n
        }
    }

    return 0
}

fn merge_counts_existing_only(
    base_counts: &mut HashMap<usize, i64>,
    new_paths: HashMap<usize, i64>,
) {
    for (position, count) in new_paths {
        

        if let Some(existing_count) = base_counts.get_mut(&position) {
            *existing_count += count;
        } else {
            base_counts.insert(position, count);
        }
        
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
