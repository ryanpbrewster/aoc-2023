use anyhow::{anyhow, bail};
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, multispace1, newline, space0},
    combinator::{all_consuming, map, map_res, value},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

/*
--- Day 19: Aplenty ---
The Elves of Gear Island are thankful for your help and send you on your way. They even have a hang glider that someone stole from Desert Island; since you're already going that direction, it would help them a lot if you would use it to get down there and return it to them.

As you reach the bottom of the relentless avalanche of machine parts, you discover that they're already forming a formidable heap. Don't worry, though - a group of Elves is already here organizing the parts, and they have a system.

To start, each part is rated in each of four categories:

x: Extremely cool looking
m: Musical (it makes a noise when you hit it)
a: Aerodynamic
s: Shiny
Then, each part is sent through a series of workflows that will ultimately accept or reject the part. Each workflow has a name and contains a list of rules; each rule specifies a condition and where to send the part if the condition is true. The first rule that matches the part being considered is applied immediately, and the part moves on to the destination described by the rule. (The last rule in each workflow has no condition and always applies if reached.)

Consider the workflow ex{x>10:one,m<20:two,a>30:R,A}. This workflow is named ex and contains four rules. If workflow ex were considering a specific part, it would perform the following steps in order:

Rule "x>10:one": If the part's x is more than 10, send the part to the workflow named one.
Rule "m<20:two": Otherwise, if the part's m is less than 20, send the part to the workflow named two.
Rule "a>30:R": Otherwise, if the part's a is more than 30, the part is immediately rejected (R).
Rule "A": Otherwise, because no other rules matched the part, the part is immediately accepted (A).
If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns. If a part is accepted (sent to A) or rejected (sent to R), the part immediately stops any further processing.

The system works, but it's not keeping up with the torrent of weird metal shapes. The Elves ask if you can help sort a few parts and give you the list of workflows and some part ratings (your puzzle input). For example:

px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
The workflows are listed first, followed by a blank line, then the ratings of the parts the Elves would like you to sort. All parts begin in the workflow named in. In this example, the five listed parts go through the following workflows:

{x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
{x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
{x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
{x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
{x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A
Ultimately, three parts are accepted. Adding up the x, m, a, and s rating for each of the accepted parts gives 7540 for the part with x=787, 4623 for the part with x=2036, and 6951 for the part with x=2127. Adding all of the ratings for all of the accepted parts gives the sum total of 19114.

Sort through all of the parts you've been given; what do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted?
*/
/*

--- Part Two ---
Even with your help, the sorting process still isn't fast enough.

One of the Elves comes up with a new plan: rather than sort parts individually through all of these workflows, maybe you can figure out in advance which combinations of ratings will be accepted or rejected.

Each of the four ratings (x, m, a, s) can have an integer value ranging from a minimum of 1 to a maximum of 4000. Of all possible distinct combinations of ratings, your job is to figure out which ones will be accepted.

In the above example, there are 167409079868000 distinct combinations of ratings that will be accepted.

Consider only your list of workflows; the list of part ratings that the Elves wanted you to sort is no longer relevant. How many distinct combinations of ratings will be accepted by the Elves' workflows?

*/
pub fn part1(input: &str) -> anyhow::Result<u32> {
    let input = parse_input(input)?;
    let wfs: HashMap<String, Workflow> = input
        .workflows
        .into_iter()
        .map(|w| (w.name.clone(), w))
        .collect();
    let accept = |item: &Item| -> anyhow::Result<bool> {
        let mut label = "in".to_owned();
        for _ in 0..wfs.len() {
            let w = wfs
                .get(&label)
                .ok_or_else(|| anyhow!("no workflow with label {label}"))?;
            match w.destination(item) {
                Destination::Accept => return Ok(true),
                Destination::Reject => return Ok(false),
                Destination::Workflow(next) => label = next,
            }
        }
        bail!("no decision after {} iterations", wfs.len())
    };
    let mut total = 0;
    for item in input.items {
        if accept(&item)? {
            total += item.kvs.values().sum::<u32>();
        }
    }
    Ok(total)
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    workflows: Vec<Workflow>,
    items: Vec<Item>,
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Item {
    kvs: HashMap<Key, u32>,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Key {
    X,
    M,
    A,
    S,
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Workflow {
    name: String,
    transitions: Vec<Transition>,
    fallback: Destination,
}
impl Workflow {
    fn destination(&self, item: &Item) -> Destination {
        for Transition {
            condition,
            destination,
        } in &self.transitions
        {
            let Some(&v) = item.kvs.get(&condition.key) else {
                continue;
            };
            let result = match condition.op {
                BinaryOp::LT => v < condition.value,
                BinaryOp::GT => v > condition.value,
            };
            if result {
                return destination.clone();
            }
        }
        self.fallback.clone()
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Transition {
    condition: Condition,
    destination: Destination,
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Condition {
    key: Key,
    op: BinaryOp,
    value: u32,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BinaryOp {
    LT,
    GT,
}
#[derive(Clone, PartialEq, Eq, Debug)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

fn parse_input(input: &str) -> anyhow::Result<Input> {
    let (_, result) = all_consuming(delimited(multispace0, input_parser, multispace0))(input)
        .map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    Ok(result)
}
fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, workflows) = workflows_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = items_parser(input)?;
    Ok((input, Input { workflows, items }))
}
fn workflows_parser(input: &str) -> IResult<&str, Vec<Workflow>> {
    separated_list1(delimited(space0, newline, space0), workflow_parser)(input)
}
fn workflow_parser(input: &str) -> IResult<&str, Workflow> {
    // gd{a>3333:R,R}
    let (input, name) = alpha1(input)?;
    let (input, (transitions, fallback)) = delimited(
        tag("{"),
        separated_pair(
            separated_list0(tag(","), transition_parser),
            tag(","),
            destination_parser,
        ),
        tag("}"),
    )(input)?;
    Ok((
        input,
        Workflow {
            name: name.to_owned(),
            transitions,
            fallback,
        },
    ))
}
fn transition_parser(input: &str) -> IResult<&str, Transition> {
    let (input, key) = key_parser(input)?;
    let (input, op) = alt((value(BinaryOp::LT, tag("<")), value(BinaryOp::GT, tag(">"))))(input)?;
    let (input, value) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, destination) = destination_parser(input)?;
    Ok((
        input,
        Transition {
            condition: Condition { key, op, value },
            destination,
        },
    ))
}
fn destination_parser(input: &str) -> IResult<&str, Destination> {
    alt((
        value(Destination::Accept, tag("A")),
        value(Destination::Reject, tag("R")),
        map(alpha1, |s: &str| Destination::Workflow(s.to_owned())),
    ))(input)
}
fn key_parser(input: &str) -> IResult<&str, Key> {
    alt((
        value(Key::X, tag("x")),
        value(Key::M, tag("m")),
        value(Key::A, tag("a")),
        value(Key::S, tag("s")),
    ))(input)
}
fn items_parser(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(delimited(space0, newline, space0), item_parser)(input)
}
fn item_parser(input: &str) -> IResult<&str, Item> {
    let (input, kvs) = delimited(tag("{"), separated_list1(tag(","), kv_parser), tag("}"))(input)?;
    Ok((
        input,
        Item {
            kvs: kvs.into_iter().collect(),
        },
    ))
}
fn kv_parser(input: &str) -> IResult<&str, (Key, u32)> {
    let (input, key) = key_parser(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, value) = map_res(digit1, str::parse)(input)?;
    Ok((input, (key, value)))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    ";

    #[test]
    fn parser_smoke_test() {
        let input = parse_input(SAMPLE_INPUT).unwrap();
        assert_eq!(input.workflows.len(), 11);
        assert_eq!(input.items.len(), 5);
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 19114);
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day19.input").unwrap()).unwrap(),
            446517,
        );
    }
}
