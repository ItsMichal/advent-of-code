use std::fs::File;
use std::io;

#[derive(Debug)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    fn get_offset(&self) -> (i32, i32) {
        match *self {
            Cardinal::North => (-1, 0),
            Cardinal::East => (0, 1),
            Cardinal::South => (1, 0),
            Cardinal::West => (0, -1),
        }
    }

    fn variants() -> Vec<Cardinal> {
        vec![
            Cardinal::North,
            Cardinal::East,
            Cardinal::South,
            Cardinal::West,
        ]
    }
}

#[derive(Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NinetyTopLeft,
    NinetyTopRight,
    NinetyBottomLeft,
    NinetyBottomRight,
}

impl Pipe {
    fn get_connecting_cardinals(&self) -> Vec<Cardinal> {
        match *self {
            Pipe::Vertical => vec![Cardinal::North, Cardinal::South],
            Pipe::Horizontal => vec![Cardinal::East, Cardinal::West],
            Pipe::NinetyTopLeft => vec![Cardinal::North, Cardinal::West],
            Pipe::NinetyTopRight => vec![Cardinal::North, Cardinal::East],
            Pipe::NinetyBottomLeft => vec![Cardinal::South, Cardinal::West],
            Pipe::NinetyBottomRight => vec![Cardinal::South, Cardinal::East],
        }
    }
}

#[derive(Debug)]
enum MapChar {
    Empty,
    Pipe(Pipe),
    Start,
}

impl MapChar {
    fn is_pipe(&self) -> bool {
        match *self {
            MapChar::Pipe(_) => true,
            _ => false,
        }
    }

    fn is_goal(&self) -> bool {
        match *self {
            MapChar::Start => true,
            _ => false,
        }
    }

    fn get_connecting(&self) -> Vec<Cardinal> {
        match self {
            MapChar::Pipe(pipe) => {
                println!("{:?} doing fre", pipe);
                pipe.get_connecting_cardinals()
            }
            MapChar::Start => Cardinal::variants(),
            MapChar::Empty => Vec::new(),
        }
    }
}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

// println!("{:?}", maps);

// let mut path = Vec::new();

#[derive(Debug)]
enum SearchResult {
    Next((i32, i32)),
    Finish,
    Error,
}

fn get_neighbors(
    current: &(i32, i32),
    map: &Vec<Vec<MapChar>>,
    explored: &mut Vec<(i32, i32)>,
) -> SearchResult {
    let neighbors: Vec<Cardinal> = map
        .get(current.0 as usize)
        .and_then(|x| x.get(current.1 as usize))
        .expect("u fuked up")
        .get_connecting();
    println!("{:?}", neighbors);
    for cardinal in neighbors {
        let offset = cardinal.get_offset();

        let new_coord = ((current.0 + offset.0), (current.1 + offset.1));
        println!("NEW {:?}", new_coord);

        match map
            .get(new_coord.0 as usize)
            .and_then(|x| x.get(new_coord.1 as usize))
        {
            Some(x) => {
                if x.is_goal() && explored.len() > 2 {
                    return SearchResult::Finish;
                } else if x.is_pipe() && !explored.contains(&new_coord) {
                    explored.push(new_coord);
                    return SearchResult::Next(new_coord);
                }
            }
            _ => {}
        }
    }
    SearchResult::Error
}

fn main() {
    let filename = "inputs/d7input.txt";

    let lines = read_lines(filename).unwrap();

    let mut map = Vec::new();

    let mut start = (0, 0);

    for line in lines.iter().enumerate() {
        let mut row = Vec::new();

        // if line.1.len() < 2 {
        //     maps.push(map);
        //     map = Vec::new();
        // }else {
        for c in line.1.chars().enumerate() {
            match c.1 {
                '.' => row.push(MapChar::Empty),
                'S' => {
                    start = (line.0 as i32, c.0 as i32);
                    // println!("Start found - {} {}", start.0, start.1);
                    row.push(MapChar::Start)
                }
                '|' => row.push(MapChar::Pipe(Pipe::Vertical)),
                '-' => row.push(MapChar::Pipe(Pipe::Horizontal)),
                'L' => row.push(MapChar::Pipe(Pipe::NinetyTopRight)),
                'J' => row.push(MapChar::Pipe(Pipe::NinetyTopLeft)),
                '7' => row.push(MapChar::Pipe(Pipe::NinetyBottomLeft)),
                'F' => row.push(MapChar::Pipe(Pipe::NinetyBottomRight)),
                _ => row.push(MapChar::Empty),
            }
        }
        // println!("{:?}", row);
        map.push(row);
        // }
    }

    let mut current = start;
    let mut explored: Vec<(i32, i32)> = Vec::new();

    let mut x = get_neighbors(&current, &map, &mut explored);
    let lmt = 100000000;
    let mut cnt = 0;
    loop {
        cnt += 1;
        if cnt > lmt {
            println!("woops");
            break;
        }
        match x {
            SearchResult::Next(newx) => {
                current = newx;
                println!("{:?}", x);
                x = get_neighbors(&current, &map, &mut explored);
            }
            SearchResult::Finish => {
                explored.push(start);
                break;
            }
            SearchResult::Error => {
                println!("what the shit!!!");
                break;
            }
        }
    }

    println!("{:?}", explored.len() / 2)
}
