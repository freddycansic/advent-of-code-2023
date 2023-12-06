fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1 = {}", part_1(input));
    println!("Part 2 = {}", part_2(input));
}

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    fn time_held_beats_record(&self, time: u64) -> bool {
        assert!(time <= self.time);

        let time_left = self.time - time;
        let velocity = time;

        velocity * time_left > self.distance
    }
}

fn part_1(input: &str) -> u64 {
    let (times, distances) = input.split_once('\n').unwrap();
    let times = times.split_once(":").unwrap().1;
    let distances = distances.split_once(":").unwrap().1;

    let records = times
        .split_whitespace()
        .zip(distances.split_whitespace())
        .map(|(time, distance)| Record {
            time: time.parse::<u64>().unwrap(),
            distance: distance.parse::<u64>().unwrap(),
        })
        .collect::<Vec<Record>>();

    records
        .into_iter()
        .map(|record| {
            (0..record.time)
                .into_iter()
                .filter(|time| record.time_held_beats_record(*time))
                .count() as u64
        })
        .fold(1, |total, next| total * next)
}

fn part_2(input: &str) -> u64 {
    let lines = input
        .split('\n')
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .split_whitespace()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();

    let record = Record {
        time: lines[0],
        distance: lines[1],
    };

    (0..record.time)
        .into_iter()
        .filter(|time| record.time_held_beats_record(*time))
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VECTOR: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(TEST_VECTOR), 288);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(TEST_VECTOR), 71503);
    }
}
