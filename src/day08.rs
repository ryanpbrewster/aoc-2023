/*
You're still riding a camel across Desert Island when you spot a sandstorm
quickly approaching. When you turn to warn the Elf, she disappears before your
eyes! To be fair, she had just finished warning you about ghosts a few minutes
ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of
documents (your puzzle input) about how to navigate the desert. At least, you're
pretty sure that's what they are; one of the documents contains a list of
left/right instructions, and the rest of the documents seem to describe some
kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the
network. Perhaps if you have the camel follow the same instructions, you can
escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel
like AAA is where you are now, and you have to follow the left/right
instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)

Starting with AAA, you need to look up the next element based on the next
left/right instruction in your input. In this example, start with AAA and go
right (R) by choosing the right element of AAA, CCC. Then, L means to choose the
left element of CCC, ZZZ. By following the left/right instructions, you reach
ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right
instructions, repeat the whole sequence of instructions as necessary: RL really
means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes
6 steps to reach ZZZ:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)

Starting at AAA, follow the left/right instructions. How many steps are required
to reach ZZZ?
*/

/*
The sandstorm is upon you and you aren't any closer to escaping the wasteland.
You had the camel follow the instructions, but you've barely left your starting
position. It's going to take significantly more steps to escape!

What if the map isn't for people - what if the map is for ghosts? Are ghosts
even bound by the laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious
fact: the number of nodes with names ending in A is equal to the number ending
in Z! If you were a ghost, you'd probably just start at every node that ends
with A and follow all of the paths at the same time until they all
simultaneously end up at nodes that end with Z.

For example:

LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)

Here, there are two starting nodes, 11A and 22A (because they both end with A).
As you follow each left/right instruction, use that instruction to
simultaneously navigate away from both nodes you're currently on. Repeat this
process until all of the nodes you're currently on end with Z. (If only some of
the nodes you're on end with Z, they act like any other node and you continue as
normal.) In this example, you would proceed as follows:

Step 0: You are at 11A and 22A.
Step 1: You choose all of the left paths, leading you to 11B and 22B.
Step 2: You choose all of the right paths, leading you to 11Z and 22C.
Step 3: You choose all of the left paths, leading you to 11B and 22Z.
Step 4: You choose all of the right paths, leading you to 11Z and 22B.
Step 5: You choose all of the left paths, leading you to 11B and 22C.
Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
So, in this example, you end up entirely on nodes that end in Z after 6 steps.

Simultaneously start on every node that ends with A. How many steps does it take
before you're only on nodes that end with Z?
*/

use std::collections::HashMap;

use anyhow::{anyhow, bail, Context};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1},
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let input = parse_input(input)?;

    let mut cur = "AAA";
    let upper_bound = input.directions.len() * input.graph.len();
    for (i, &dir) in input
        .directions
        .iter()
        .cycle()
        .enumerate()
        .take(upper_bound)
    {
        if cur == "ZZZ" {
            return Ok(i);
        }
        let Some((l, r)) = input.graph.get(cur) else {
            bail!("no transition away from {cur}");
        };
        cur = match dir {
            Direction::Left => l,
            Direction::Right => r,
        };
    }
    bail!("did not reach an exit, giving up after {upper_bound} steps");
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let input = parse_input(input)?;

    // Regardless of where you start, you'll eventually end up in a cycle.  By
    // manually playing with the input I have determined that, each starting
    // position goes through exactly one __Z node during its cycle, and none
    // of the starting positions hit a __Z node before entering a cycle.
    // That makes this problem enormously simpler.

    // This implementation is intentionally very brittle. If you feed it an input
    // that doesn't have those two characteristics, it will explode rather than spit
    // out an incorrect answer.

    let cycle_lengths = input
        .graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| compute_cycle(start, &input.directions, &input.graph))
        .collect::<anyhow::Result<Vec<_>>>()?;

    // By virtue of how `compute_cycle` works, we now have a list of effective cycle lengths, l_i, where we are looking for
    // some time `t > 0` such that
    //   t === 0 (mod l_0) AND t === 0 (mod l_1) AND ...
    // which can be solved by just finding any common multiple of {l_0, ..., l_k}.
    // Here we'll use LCM to find the smallest.

    // I think that if we had a slightly less pleasant input, we would get congruences like
    //   t === i_0 (mod l_0) AND t === i_1 (mod l_1) AND ...
    // which would effectively represent cycles with some "lead in" time.

    // An even less pleasant input would result in congruences like
    //   [ t === i_0 (mod l_0) OR t === j_0 (mod l_0) OR t === k_0 (mod l_0) ]
    //         AND
    //   [ t === i_1 (mod l_1) OR t === j_1 (mod l_1) ]
    //         AND
    //         ...
    // which would effectively represent cycles that pass through multiple non-uniformly-distributed
    // __Z nodes (e.g., AAA -> 11Z -> 22Z -> AAA, where you're at a __Z node for t = 1 (mod 3) or t = 2 (mod 3), but at t = 0 (mod 3) you're on AAA)

    cycle_lengths
        .iter()
        .copied()
        .reduce(lcm)
        .ok_or(anyhow!("graph has no __A nodes"))
}

fn compute_cycle(
    start: &str,
    directions: &[Direction],
    graph: &HashMap<String, (String, String)>,
) -> anyhow::Result<usize> {
    let mut cur = start;

    // In order to tell whether we're actually in a cycle, we need to have been at the same node,
    // and in the same position in our directions loop.
    let mut vis: HashMap<&str, HashMap<usize, usize>> = HashMap::new();
    let mut iter = directions.iter().cycle().enumerate();
    loop {
        let (i, &dir) = iter.next().unwrap();
        let pos = i % directions.len();
        if let Some(&prev) = vis.get(cur).and_then(|v| v.get(&pos)) {
            // We hit a cycle!
            let cycle_length = i - prev;
            let winners: Vec<usize> = vis
                .into_iter()
                .filter(|(k, _v)| k.ends_with('Z'))
                .flat_map(|(_k, v)| v.into_values())
                .collect();
            let effective_cycle_length = winners
                .iter()
                .copied()
                .reduce(gcd)
                .context("the cycle must contain __Z nodes")?;
            if effective_cycle_length * winners.len() != cycle_length {
                bail!("we ony handle cases where the exit condition can be expressed as `t = 0 (mod m)`, and a cycle of {cycle_length} w/ __Z nodes at {winners:?} cannot");
            }
            return Ok(effective_cycle_length);
        }
        vis.entry(cur).or_default().insert(pos, i);
        cur = match dir {
            Direction::Left => graph.get(cur).unwrap().0.as_str(),
            Direction::Right => graph.get(cur).unwrap().1.as_str(),
        };
    }
}

fn gcd(m: usize, n: usize) -> usize {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}
fn lcm(m: usize, n: usize) -> usize {
    m / gcd(m, n) * n
}

struct Input {
    directions: Vec<Direction>,
    graph: HashMap<String, (String, String)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> anyhow::Result<Input> {
    let (_, result) = all_consuming(delimited(multispace0, input_parser, multispace0))(input)
        .map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    Ok(result)
}
fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, (directions, graph)) =
        separated_pair(directions_parser, multispace1, graph_parser)(input)?;
    Ok((input, Input { directions, graph }))
}

fn directions_parser(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
    )))(input)
}

fn graph_parser(input: &str) -> IResult<&str, HashMap<String, (String, String)>> {
    let (input, edges) = separated_list1(multispace1, edge_parser)(input)?;
    Ok((input, edges.into_iter().collect()))
}
fn edge_parser(input: &str) -> IResult<&str, (String, (String, String))> {
    let (input, src) = alphanumeric1(input)?;
    let (input, _) = delimited(multispace1, tag("="), multispace1)(input)?;
    let (input, (left, right)) = delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    )(input)?;
    Ok((input, (src.to_owned(), (left.to_owned(), right.to_owned()))))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    ";

    #[test]
    fn parser_smoke_test() {
        let parsed = parse_input(SAMPLE_INPUT).unwrap();
        assert_eq!(parsed.directions, vec![Direction::Right, Direction::Left]);
        assert_eq!(parsed.graph.len(), 7);
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 2);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day08.input").unwrap()).unwrap(),
            12737,
        );
    }

    const SAMPLE_INPUT_2: &str = "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    ";

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT_2).unwrap(), 6);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day08.input").unwrap()).unwrap(),
            9064949303801,
        );
    }

    #[test]
    fn fuzz_artifact_1() {
        assert!(part1(
            "
            R     AAA     =

            (x, R)
            
            =       R 
            (x, R)           
        "
        )
        .is_err());
    }
}
