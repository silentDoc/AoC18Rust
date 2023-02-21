pub fn solve(lines:Vec<String>, part:i32) -> i32
{
    return solve_part_1(lines);
}

fn check_num_letters(input:String, target:usize) -> i32
{
    let char_arr:Vec<char> = input.chars().collect();
    for c in char_arr
    {
        let occurrences = input.matches(c).count();
        if occurrences==target
        {
            return 1;
        }
    }
    return 0;
}

fn solve_part_1(lines:Vec<String>) -> i32
{
    let count_2_letters:i32 = lines.iter()
                                    .map(|x| check_num_letters(x.to_string(), 2))
                                    .sum();
    let count_3_letters:i32 = lines.iter()
                                .map(|x| check_num_letters(x.to_string(), 3))
                                .sum();

    return count_2_letters * count_3_letters;
}