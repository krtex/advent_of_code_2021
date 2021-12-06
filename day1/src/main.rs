fn part1()
{
    let mut count  = 0;
    let input = include_str!("input").lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u16>>();

    for elem in input.windows(2)
    {
        if elem[0] < elem[1]
        {
            count += 1;
        }
    }
    println!("Part one: {}", count);
}

fn part2()
{
    let mut count  = 0;
    let input = include_str!("input").lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u16>>()
        .windows(3).map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<u16>>();

    for elem in input.windows(2)
    {
        if elem[0] < elem[1]
        {
            count += 1;
        }
    }
    println!("Part two: {}", count);
}

fn main()
{
    part1();
    part2();
}
