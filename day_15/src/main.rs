use std::time::Instant;



use helpers::AocArgs;

fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    // let input_txt = include_str!("../input-demo.txt");

    let (eval_level, max_axis) = if args.demo_text {
        (10, 20)
    } else {
        (2_000_000, 4000000)
    };

    // Parse the input into a Sensor

    let sensors: Vec<Sensor> = input_txt
        .lines()
        .map(|line| {
            // split each line and only grab what we want
            let mut numbers = line
                .split(&['=', ',', ':'])
                .filter_map(|num| num.parse::<i32>().ok());
            // sensor coordinate will always be the first two numbers in the iterator
            let sensor = Point::new(numbers.next().unwrap(), numbers.next().unwrap());
            let beacon = Point::new(numbers.next().unwrap(), numbers.next().unwrap());

            // Calculate the manhattan distance for the two points
            let distance = manhattan_distance(&sensor, &beacon);

            Sensor::new(&sensor, distance, &beacon)
        })
        .collect();

    if args.part_one {
        // Now that we have an iter of Sensors we can see if the sensors cover the target location
        let (from_max, to_max) =
            sensors
                .iter()
                .fold((i32::MAX, i32::MIN), |(from, to), sensor| -> (i32, i32) {
                    let (s_min, s_max) = sensor.get_coverage(eval_level);

                    if s_min != 0 && s_max != 0 {
                        println!(
                            "Coverage of (from {s_min} to {s_max}) for sensor at ({}, {})",
                            sensor.location.x, sensor.location.y
                        );
                        //the sensor covers the target
                        println!("current from value: {from}");
                        println!("min of from value and s_max: {}", from.min(s_min));
                        (from.min(s_min), to.max(s_max))
                    } else {
                        (from, to)
                    }
                });
        println!("min {from_max} max {to_max}");

        let solution = from_max.abs() + to_max;

        println!("solution: {solution}");
    }

    if args.part_two {
        let distress_beacon= find_distress_signal(&sensors, max_axis);
        println!("Distress beacon found at ({}, {})", distress_beacon.0, distress_beacon.1);
        let solution = distress_beacon.0 as i64 * 4000000  + distress_beacon.1 as i64;
        println!("solution: {solution}");
    }

    println!("executed in {}mc", now.elapsed().as_micros());
}

struct Sensor {
    location: Point,
    distance_to_beacon: i32,
    beacon_location: Point,
}

impl Sensor {
    pub fn new(location: &Point, distance: i32, beacon: &Point) -> Self {
        Self {
            location: location.to_owned(),
            distance_to_beacon: distance,
            beacon_location: beacon.to_owned(),
        }
    }

    fn get_coverage(&self, target: i32) -> (i32, i32) {
        // get min y sensor covers and the max y sensor covers
        let min_y = self.location.y - self.distance_to_beacon;
        let max_y = self.location.y + self.distance_to_beacon;

        if target > min_y && target < max_y {
            // we cover the target zone
            // determine how much of the zone we cover
            let min_x =
                self.location.x - (self.distance_to_beacon - (target - self.location.y).abs());
            let max_x =
                self.location.x + (self.distance_to_beacon - (target - self.location.y).abs());
            (min_x, max_x)
        } else {
            (0, 0)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Finds the distress signal. This is done by finding the point that all sensors 
/// don't cover within our max_axis
fn find_distress_signal(sensors: &[Sensor], max_axis: i32) -> (i32, i32) {
    println!("max_axis: {max_axis}");
    for y in 0..=max_axis {
        let mut x = 0;
    
        'find_x: while x <= max_axis {
            for sensor in sensors {
                let (min_x, max_x) = sensor.get_coverage(y);
                if (min_x..max_x).contains(&x) {
                    x = max_x + 1;
                    continue 'find_x;
                }
            }
            break;
        }
        if x <= max_axis {
            return(x, y)
        }
    }
    (0,0)
}

fn manhattan_distance(point_a: &Point, point_b: &Point) -> i32 {
    (point_a.x - point_b.x).abs() + (point_a.y - point_b.y).abs()
}

#[cfg(test)]
mod tests {
    use crate::{manhattan_distance, Point};

    #[test]
    fn get_manhattan_distance() {
        let sensor = Point::new(8, 7);
        let beacon = Point::new(2, 10);

        println!("{}", manhattan_distance(&sensor, &beacon));
    }
}