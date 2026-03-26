
const DIAL: i32 = 100;
const START_POS: i32 = 50;

fn parse_line(raw: &str) -> Option<(char, i32)> {
    let line = raw.trim().trim_start_matches('\u{feff}');
    if line.is_empty() {
        return None;
    }
    if line.len() < 2 {
        panic!("Instruction trop courte: {:?}", line);
    }

    let (d, v) = line.split_at(1);
    let dir = d.chars().next().unwrap();
    let steps: i32 = v
        .trim()
        .parse()
        .unwrap_or_else(|e| panic!("Nombre invalide dans {:?}: {}", line, e));

    Some((dir, steps))
}


fn count_hits_zero_during_move(pos: i32, dir: char, steps: i32) -> usize {
    if steps <= 0 {
        return 0;
    }

    let pos = pos.rem_euclid(DIAL);


    let t_first = match dir {
        'R' => if pos == 0 { DIAL } else { DIAL - pos },
        'L' => if pos == 0 { DIAL } else { pos },
        _ => panic!("Direction invalide: {:?}", dir),
    };

    if steps < t_first {
        0
    } else {

        (1 + (steps - t_first) / DIAL) as usize
    }
}

pub fn p1(input: &str) -> usize {
    let mut pos: i32 = START_POS;
    let mut stops: usize = 0;

    for raw in input.lines() {
        let Some((dir, steps)) = parse_line(raw) else { continue };

        match dir {
            'R' => pos = (pos + steps).rem_euclid(DIAL),
            'L' => pos = (pos - steps).rem_euclid(DIAL),
            _ => panic!("Direction invalide: {:?}", dir),
        }

        if pos == 0 {
            stops += 1;
        }
    }

    stops
}

pub fn p2(input: &str) -> usize {
    let mut pos: i32 = START_POS;
    let mut passes: usize = 0;

    for raw in input.lines() {
        let Some((dir, steps)) = parse_line(raw) else { continue };

        passes += count_hits_zero_during_move(pos, dir, steps);

        // mise à jour position finale
        match dir {
            'R' => pos = (pos + steps).rem_euclid(DIAL),
            'L' => pos = (pos - steps).rem_euclid(DIAL),
            _ => panic!("Direction invalide: {:?}", dir),
        }
    }

    passes
}

#[cfg(test)]
mod test {
    
    use crate::day1::p1;
    #[test]
    fn p1_test(){
        let input =  include_str!("day_test.txt");
        assert_eq!(p1(input), 3);
    }
}