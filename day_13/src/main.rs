use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::time::Instant;
use serde::Deserialize;
use helpers::AocArgs;

///-  If both values are integers, the lower integer should come first. If the left integer is lower than
///  the right integer, the inputs are in the right order. If the left integer is higher than the right
/// integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input.
/// - If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
/// - If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].
fn main() {
    let now = Instant::now();
    let args: AocArgs = argh::from_env();

    let input_txt = if args.demo_text {
        include_str!("../input-demo.txt")
    } else {
        include_str!("../input.txt")
    };

    // let input_txt =  include_str!("../input-demo.txt");

    // println!("PACKET PAIRS: {:?}", &packet_pairs);

    if args.part_two {
        let divider_packet_1: PacketData = serde_json::from_str("[[2]]").unwrap();
        let divider_packet_2: PacketData = serde_json::from_str("[[6]]").unwrap();
        let mut packets: Vec<PacketData> = input_txt
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_str::<PacketData>(line).unwrap())
            .collect();
        packets.push(divider_packet_1.clone());
        packets.push(divider_packet_2.clone());

        packets.sort_by(|left, right| match PacketPair::is_in_correct_order(left, right) {
            None => Equal,
            Some(true) => Less,
            Some(false) => Greater,
        });

        let solution: usize = packets
            .iter()
            .zip(1_usize..)
            .filter_map(|(packet, id)|{
                if packet == &divider_packet_1 || packet == &divider_packet_2 {
                    Some(id)
                } else {
                    None
                }
            })
            .product();

        println!("SOLUTION: {solution}")

    } else {
    let packet_pairs: Vec<PacketPair> = input_txt
        .split("\n\n")
        .filter_map(|a| a
            .split_once("\n")
            .map(|(l, r)| PacketPair::new(l, r))

        ).collect();
        let mut ordered_indexes = vec![];
        for (packet_pair, index) in packet_pairs.iter().zip(1_usize..) {
            let in_order = PacketPair::is_in_correct_order(&packet_pair.left, &packet_pair.right).unwrap();
            println!("Index: {index} | IS Ordered: {:?} | Orig Pair: {:?}", &in_order, &packet_pair);

            if in_order{ordered_indexes.push(index)}
        }


        println!("Pairs in correct order: {:?}", ordered_indexes);
        let sum: usize = ordered_indexes.iter().sum();

        println!("SOLUTION: {sum}");
    }


    println!("executed in {}mc", now.elapsed().as_micros());
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(u8),
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PacketPair {
    left: PacketData,
    right: PacketData,
}

impl PacketPair {
    fn new(left: &str, right: &str) -> Self {
        Self {
            left: serde_json::from_str(left).unwrap(),
            right: serde_json::from_str(right).unwrap(),
        }
    }

    fn is_in_correct_order(left: &PacketData, right: &PacketData) -> Option<bool> {
        match (left, right) {
            (PacketData::Integer(l), PacketData::Integer(r)) => match l.cmp(r) {
                Ordering::Less => Some(true),
                Ordering::Equal => None,
                Ordering::Greater => Some(false),
            },
            (PacketData::List(l), PacketData::List(r)) => {
                if l.is_empty() && !r.is_empty() {
                    return Some(true);
                }

                if r.is_empty() && !l.is_empty() {
                    return Some(false);
                }

                let min_length = l.len().min(r.len());


                let result = (0..min_length).fold(None, |acc, id| acc.or(PacketPair::is_in_correct_order(&l[id], &r[id])));
                result.or_else(|| match l.len().cmp(&r.len()) {
                        Ordering::Less => Some(true),
                        Ordering::Equal => None,
                        Ordering::Greater => Some(false),
                    })

            }
            (l @ PacketData::List(_), r @ PacketData::Integer(_)) => PacketPair::is_in_correct_order(l, &PacketData::List(vec![r.clone()])),
            (l @ PacketData::Integer(_), r @ PacketData::List(_)) => PacketPair::is_in_correct_order(&PacketData::List(vec![l.clone()]), r),
        }
    }
}

