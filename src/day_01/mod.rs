use std::collections::HashSet;

pub fn solve(lines:Vec<String>, part:i32) -> i32
{
    if part ==1
    {
        return parse_input(lines).iter().sum();
    }
    return find_first_repeat(lines);
}

fn find_first_repeat(lines:Vec<String>) -> i32
{
    let freqs = parse_input(lines);
    
    let mut known_freqs: HashSet<i32> = HashSet::new();
    let mut current_freq:i32 = 0;

    let mut current_index:usize = 0;
    let freq_count = freqs.len();

    while known_freqs.insert(current_freq)
    {
        current_freq += freqs[current_index];
        current_index = (current_index+1) % freq_count;
    }
    return current_freq;
}


fn parse_input(lines:Vec<String>) -> Vec<i32> 
{ 
    let ret_val = lines.iter().flat_map(|line| line.parse::<i32>()).collect();
    return ret_val;
}
