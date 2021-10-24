use super::utils::ParseError;

#[derive(Debug)]
struct Terminal {
    timestamp: usize,
    busses: Vec<Option<usize>>,
}

fn parse_input() -> Result<Terminal, ParseError> {
    let input = include_str!("./data/input.txt");
    let lines = input
        .lines()
        .filter(|v| *v != "")
        .collect::<Vec<_>>();

    let timestamp = lines[0].parse::<usize>()?;
    let busses = lines[1]
        .split(',')
        .map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();

    Ok(Terminal { timestamp, busses })
}

pub fn problem1() -> Result<(), ParseError> {
    let terminal = parse_input()?;

    let mut next_arrivals = terminal.busses.iter()
        .filter(|b| b.is_some())
        .map(|b| b.unwrap())
        .map(|b| (b, (terminal.timestamp - (terminal.timestamp % b)) + b))
        .collect::<Vec<_>>();

    next_arrivals.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let next = next_arrivals.iter().next().ok_or(ParseError::new("No bus found"))?;

    let arrives_in = next.1 - terminal.timestamp;
    println!("Next arriving bus is {} in {} minutes.", next.0, arrives_in);
    println!("13/1: checksum is {}", next.0 * arrives_in);

    Ok(())
}

struct Crt {
    remainder: usize,
    modulus: usize,
}

fn chinese_remainder(crts: &Vec<Crt>) -> usize {
    // search the solution with the chinese remainder theorem
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving

    let mut time = crts[0].remainder;
    let mut increment = crts[0].modulus;

    for crt in crts.iter().skip(1) {
        loop {
            if time % crt.modulus == crt.remainder {
                break;
            }

            time += increment;
        }

        increment *= crt.modulus;
    }

    time
}

pub fn problem2() -> Result<(), ParseError> {
    let terminal = parse_input()?;

    let mut busses = terminal.busses.into_iter()
        .enumerate()
        // Filter each irrelevant bus
        .filter(|(_, b)| b.is_some())

        // The timestamp t we are looking for has to fulfill the equations
        //
        //   t + offset_i mod bus_i = 0
        //
        // but the CRT is giving us instructions on how to solve
        //
        //   t mod bus_i = a_i
        //
        // for a_i with
        //
        //   0 <= a_i < bus_i
        //
        // So we have to get the a_i from our input. This is easy but we have
        // to consider a few corner cases. In case
        //
        //   0 < offset_i < bus_i
        //
        // we get
        //
        //   a_i = bus_i - offset_i
        //
        // We now have to consider two special cases:
        //
        //   offset_i = 0
        //
        // and
        //
        //   offset_i > bus_id
        //
        // To eliminate the first one we can apply the modulus operation on the
        // difference:
        //
        //   (bus_i - offset_i) % bus_i
        //
        // To eliminate the second issue, we can apply the modulus operation on
        // the offset directly:
        //
        //   (bus_i - offset_i % bus_i)
        //
        // Putting everything together we now have:
        //
        //   a_i = (bus_i - offset_i % bus_i) % bus_i
        //
        // which is exactly how we are calculating the first component of
        // belows result tuple:
        .map(|(i, b)| ((b.unwrap() - i % b.unwrap()) % b.unwrap(), b.unwrap()))
        .map(|(remainder, modulus)| Crt { remainder, modulus })
        .collect::<Vec<_>>();

    // sort descending by bus id
    busses.sort_by(|a, b| b.modulus.partial_cmp(&a.modulus).unwrap());

    let result = chinese_remainder(&busses);
    println!("13/2: result {}", result);

    Ok(())
}