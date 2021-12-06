use std::str::FromStr;
use std::num::ParseIntError;

struct Position
{
    horizontal : u32,
    depth : u32,
}
enum Direction
{
    Forward, Up, Down
}

struct Command
{
    direction: Direction,
    distance: u32,
}

impl FromStr for Command
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let parts: Vec<&str> = s.split(' ')
                                .collect();

        let dir = match parts[0]
        {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => Direction::Forward,
        };
        let dis = parts[1].parse::<u32>()?;

        Ok(Command { direction: dir, distance: dis })
    }
}

fn part1()
{
    let mut pos = Position {horizontal: 0, depth: 0};
    let input = include_str!("input").lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Command>>();
    
    for comm in input
    {
        match comm.direction
        {
            Direction::Forward => pos.horizontal += comm.distance,
            Direction::Up => pos.depth -= comm.distance,
            Direction::Down => pos.depth += comm.distance,
        }
    }

    println!("Position is {}, {}, and {}", pos.horizontal, pos.depth, pos.horizontal*pos.depth);
}

fn part2()
{
    let mut pos = Position {horizontal: 0, depth: 0};
    let mut aim = 0;
    let input = include_str!("input").lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Command>>();
    
    for comm in input
    {
        match comm.direction
        {
            Direction::Forward =>
            {
                pos.horizontal += comm.distance;
                pos.depth += comm.distance * aim;
            },
            Direction::Up => aim -= comm.distance,
            Direction::Down => aim += comm.distance,
        }
    }

    println!("Position is {}, {}, and {}", pos.horizontal, pos.depth, pos.horizontal*pos.depth);
}
fn main()
{
    part1();
    part2();
}
