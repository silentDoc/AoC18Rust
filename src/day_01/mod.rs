pub fn solve(lines:Vec<String>, _part:i32) -> i32
{
    return parse_input(lines).iter().sum();
}

fn parse_input(lines:Vec<String>) -> Vec<i32> 
{ 
    let ret_val = lines.iter().flat_map(|line| line.parse::<i32>()).collect();
    return ret_val;
}
