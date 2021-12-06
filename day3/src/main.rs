fn part1()
{
    let input = include_str!("input").lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>();
    
    let gamma: [i16; 12] = [0; 12];

    for line in &input
    {
        for (i, c) in line.enumerate()
        {
            match c
            {
                "0" => gamma[i] -= 1,
                "1" => gamma[i] += 1,
            } 
        }

    }
}

fn main()
{
    part1();
}
