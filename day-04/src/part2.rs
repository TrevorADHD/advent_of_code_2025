#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut lines: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
    let line_len = lines.len();
    let append_line = vec!['.'; line_len];
    lines.push(append_line);

    loop {

        let ret = process_roll_and_roll( line_len, &mut result, &mut lines );
        if ret == 0 {
            break;
        }
    }

    Ok(result.to_string())
}

fn process_roll_and_roll(line_len:usize , result: &mut u32, lines: &mut Vec<Vec<char>>) -> u32 {
    let mut pos_arr: Vec<(usize, usize)> = vec![]; 
    let (mut cur_arr, mut next_arr): (Vec<u32>, Vec<u32>) = (vec![0; line_len], vec![0; line_len]);

    let mut total_count = 0;
    for n in 0..line_len {
        let count = traversal(&lines[n], &lines[n + 1], &mut cur_arr, &mut next_arr, n, &mut pos_arr);
        *result = *result + count;
        total_count += count;
        cur_arr = next_arr;
        next_arr = vec![0; line_len];
    }
    // gen_next_rolls(&mut lines, &pos_arr);
    gen_next_rolls(lines, &pos_arr);
    // println!("pos_arr: {:?}", pos_arr);
    // println!("new lines: {:?}", lines);
    total_count
}

fn gen_next_rolls(lines : &mut Vec<Vec<char>>, pos_arr: &Vec<(usize, usize)>){
    for (x, y) in pos_arr {
        lines[*x][*y] = '.'
    }
}

   
fn neighbor_and_self(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>, line_index: usize) {
    right(p, cur, next, cur_arr, next_arr,line_index);
    left_down(p, cur, next, cur_arr, next_arr, line_index);
    down(p, cur, next, cur_arr, next_arr, line_index);
    right_down(p, cur, next, cur_arr, next_arr, line_index);
}

fn right(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>, line_index: usize) {
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

fn down(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>, line_index: usize) {
    let arr_len = cur_arr.len();
    if next[p] == '.' {
        return 
    }

    next_arr[p] = next_arr[p] + 1;
    cur_arr[p] = cur_arr[p] + 1;
}

fn right_down(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>, line_index: usize) {
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

fn left_down(p: usize , cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>, line_index: usize) {
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

fn traversal(cur: &Vec<char>, next:&Vec<char>, cur_arr: &mut Vec<u32> , next_arr: &mut Vec<u32>, line_index: usize, pos_arr: &mut Vec<(usize, usize)>) -> u32 {
    let arr_len = cur_arr.len();
    let mut count = 0;
    for n in 0..arr_len {
        if cur[n] == '.' { continue }
        neighbor_and_self(n, cur, next, cur_arr, next_arr, line_index);
        if cur_arr[n] < THRESHOLD {
            count = count + 1;
            pos_arr.push((line_index, n));
        }
        // println!("cur_arr : {:?}", cur_arr);
        // println!("next_arr: {:?}", next_arr);
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
