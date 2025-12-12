use itertools::{max};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let lines = _input.lines().map(|line| {
        line.split(' ').collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();

    let indicators = build_indicator(&lines);
    let buttons = build_buttons(&lines);
    let joltage = build_joltage(&lines);

    // println!("{:?} , {:?}, {:?}", indicators, buttons, joltage);

    let mut result = 0;

    for n in 0..indicators.len() {
        let indicator_chars = indicators[n].chars().collect::<Vec<char>>();
        let buttons_slice = buttons[n];
        // let joltage_num = joltage[n].parse::<u64>().unwrap();

        let min_steps = get_min_steps(indicator_chars, buttons_slice);
        println!("min steps for indicator {}",  min_steps);
        result = result + min_steps ;
    }

    Ok(result.to_string())
}

fn get_min_steps(indicator: Vec<char>, button: &[&str]) -> u64 {
    use std::collections::{VecDeque, HashSet};
    
    let mut buttons_num = vec![];
    for b in button {
        let t = b[1..b.len() - 1].split(',').map(|num| num.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        buttons_num.push(t);
    }

    // BFS to find minimum steps
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    // Start from indicator state
    queue.push_back((indicator.clone(), 0u64));
    visited.insert(indicator.clone());
    
    while let Some((current, steps)) = queue.pop_front() {
        // Check if we reached all '.' (goal state)
        if current.iter().all(|&c| c == '.') {
            return steps;
        }
        
        // Try all buttons
        for b in &buttons_num {
            let new_state = switch(&current, b);
            
            if !visited.contains(&new_state) {
                visited.insert(new_state.clone());
                queue.push_back((new_state, steps + 1));
            }
        }
    }
    
    // If we can't reach the target
    u64::MAX
}



fn switch (indicator: &Vec<char> , pos_list: &Vec<usize>) -> Vec<char> {
    let mut result: Vec<char> =  indicator.clone();
    for n in pos_list {
        if result[*n] == '.' {
            result[*n] = '#'
        } else {
            result[*n] = '.'
        }
    }

    result
}

fn build_indicator<'a>(lines: &Vec<Vec<&'a str>>) -> Vec<&'a str> {

    let mut indicators = vec![];
    for line in lines {
        indicators.push(line[0][1..line[0].len() - 1].trim());
    }
    indicators
}

fn build_joltage<'a>(lines: &Vec<Vec<&'a str>>) -> Vec<&'a str> {
    let mut joltages = vec![];
    for line in lines {
        if let Some(&last) = line.last() {
            joltages.push(last);
        }
    }
    joltages
}

fn build_buttons<'a>(lines: &'a Vec<Vec<&'a str>>) -> Vec<&'a [&'a str]> {
    let mut buttons = vec![];
    for line in lines {
        buttons.push(&line[1..line.len() - 1]);
    }

    buttons
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
