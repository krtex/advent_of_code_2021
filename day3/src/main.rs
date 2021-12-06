fn part1()
{
    let input = include_str!("input").lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>();
    
    let mut counts: [i16; 12] = [0; 12];

    for line in &input
    {
        for (i, c) in line.chars().enumerate()
        {
            match c
            {
                '0' => counts[i] -= 1,
                '1' => counts[i] += 1,
                _ => counts[i] += 0,
            } 
        }
    }
    
    let mut gamma = 0;
    for i in 0..counts.len()
    {
        if counts[i] > 0
        {
            gamma +=  1 << (11 - i);
        }
    }
    println!("Gamma = {}, gamma*epsilon = {}",
    gamma, gamma * (!gamma & ((1 << 12) - 1)));
}

fn main()
{
    part1();
}
