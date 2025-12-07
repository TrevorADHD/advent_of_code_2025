#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut lines: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
    let line_len = lines.len();
    let append_line = vec!['.'; line_len];
    lines.push(append_line);
    let (mut cur_arr, mut next_arr): (Vec<u32>, Vec<u32>) = (vec![0; line_len], vec![0; line_len]);

    for n in 0..line_len {
        result = result + traversal(&lines[n], &lines[n + 1], &mut cur_arr, &mut next_arr);
        cur_arr = next_arr;
        next_arr = vec![0; line_len];
    }
    
    // // traversal last line
    // for (n, c) in lines[line_len -1].iter().enumerate() {
    //     if *c == '.' { continue } 
    //     else {
    //         if cur_arr[n] < THRESHOLD {
    //             result = result + 1;
    //         }
    //     }
    // }
    Ok(result.to_string())
}

   
fn neighbor_and_self(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>) {
    right(p, cur, next, cur_arr, next_arr);
    left_down(p, cur, next, cur_arr, next_arr);
    down(p, cur, next, cur_arr, next_arr);
    right_down(p, cur, next, cur_arr, next_arr);
}

fn right(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>) {
    let arr_len = cur_arr.len();
    if p + 1 > arr_len - 1 {
        return 
    } else {
        if cur[p + 1] == '.' {return }
        else {
            cur_arr[p + 1]  = cur_arr[p + 1] + 1;
            cur_arr[p] = cur_arr[p] + 1;
        }
    }

}

fn down(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>) {
    let arr_len = cur_arr.len();
    if next[p] == '.' {
        return 
    }

    next_arr[p] = next_arr[p] + 1;
    cur_arr[p] = cur_arr[p] + 1;
}

fn right_down(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>) {
    let arr_len = cur_arr.len();
    if p + 1 > arr_len - 1 {
        return 
    } else {
        if next[p + 1] == '.' {return }
        else {
            next_arr[p + 1]  = next_arr[p + 1] + 1;
            cur_arr[p] = cur_arr[p] + 1;
        }
    }
}

fn left_down(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>) {
    let arr_len = cur_arr.len();
    if p ==  0 {
        return 
    } else {
        if next[p - 1] == '.' {return }
        else {
            next_arr[p - 1]  = next_arr[p - 1] + 1;
            cur_arr[p] = cur_arr[p] + 1;
        }
    }
}

fn traversal(cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>) -> u32 {
    let arr_len = cur_arr.len();
    let mut count = 0;
    for n in 0..arr_len {
        if cur[n] == '.' { continue }
        neighbor_and_self(n, cur, next, cur_arr, next_arr);
        if cur_arr[n] < THRESHOLD {
            count = count + 1;
        }
        println!("cur_arr : {:?}", cur_arr);
        println!("next_arr: {:?}", next_arr);
    }
    count
}

const THRESHOLD:u32 = 4;

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
