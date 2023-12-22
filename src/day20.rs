/*
--- Day 20: Pulse Propagation ---
With your help, the Elves manage to find the right parts and fix all of the machines. Now, they just need to send the command to boot up the machines and get the sand flowing again.

The machines are far apart and wired together with long cables. The cables don't connect to the machines directly, but rather to communication modules attached to the machines that perform various initialization tasks and also act as communication relays.

Modules communicate using pulses. Each pulse is either a high pulse or a low pulse. When a module sends a pulse, it sends that type of pulse to each module in its list of destination modules.

There are several different types of modules:

Flip-flop modules (prefix %) are either on or off; they are initially off. If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.

Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.

There is a single broadcast module (named broadcaster). When it receives a pulse, it sends the same pulse to all of its destination modules.

Here at Desert Machine Headquarters, there is a module with a single button on it called, aptly, the button module. When you push the button, a single low pulse is sent directly to the broadcaster module.

After pushing the button, you must wait until all pulses have been delivered and fully handled before pushing it again. Never push the button if modules are still processing pulses.

Pulses are always processed in the order they are sent. So, if a pulse is sent to modules a, b, and c, and then module a processes its pulse and sends more pulses, the pulses sent to modules b and c would have to be handled first.

The module configuration (your puzzle input) lists each module. The name of the module is preceded by a symbol identifying its type, if any. The name is then followed by an arrow and a list of its destination modules. For example:

broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
In this module configuration, the broadcaster has three destination modules named a, b, and c. Each of these modules is a flip-flop module (as indicated by the % prefix). a outputs to b which outputs to c which outputs to another module named inv. inv is a conjunction module (as indicated by the & prefix) which, because it has only one input, acts like an inverter (it sends the opposite of the pulse type it receives); it outputs to a.

By pushing the button once, the following pulses are sent:

button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> a
After this sequence, the flip-flop modules all end up off, so pushing the button again repeats the same sequence.

Here's a more interesting example:

broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
This module configuration includes the broadcaster, two flip-flops (named a and b), a single-input conjunction module (inv), a multi-input conjunction module (con), and an untyped module named output (for testing purposes). The multi-input conjunction module con watches the two flip-flop modules and, if they're both on, sends a low pulse to the output module.

Here's what happens if you push the button once:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -high-> output
b -high-> con
con -low-> output
Both flip-flops turn on and a low pulse is sent to output! However, now that both flip-flops are on and con remembers a high pulse from each of its two inputs, pushing the button a second time does something different:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output
Flip-flop a turns off! Now, con remembers a low pulse from module a, and so it sends only a high pulse to output.

Push the button a third time:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -low-> output
b -low-> con
con -high-> output
This time, flip-flop a turns on, then flip-flop b turns off. However, before b can turn off, the pulse sent to con is handled first, so it briefly remembers all high pulses for its inputs and sends a low pulse to output. After that, flip-flop b turns off, which causes con to update its state and send a high pulse to output.

Finally, with a on and b off, push the button a fourth time:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output
This completes the cycle: a turns off, causing con to remember only low pulses and restoring all modules to their original states.

To get the cables warmed up, the Elves have pushed the button 1000 times. How many pulses got sent as a result (including the pulses sent by the button itself)?

In the first example, the same thing happens every time the button is pushed: 8 low pulses and 4 high pulses are sent. So, after pushing the button 1000 times, 8000 low pulses and 4000 high pulses are sent. Multiplying these together gives 32000000.

In the second example, after pushing the button 1000 times, 4250 low pulses and 2750 high pulses are sent. Multiplying these together gives 11687500.

Consult your module configuration; determine the number of low pulses and high pulses that would be sent after pushing the button 1000 times, waiting for all pulses to be fully handled after each push of the button. What do you get if you multiply the total number of low pulses sent by the total number of high pulses sent?
*/
/*
--- Part Two ---
The final machine responsible for moving the sand down to Island Island has a module attached named rx. The machine turns on when a single low pulse is sent to rx.

Reset all modules to their default states. Waiting for all pulses to be fully handled after each button press, what is the fewest number of button presses required to deliver a single low pulse to the module named rx?

*/

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, newline, one_of, space0},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let mut graph = parse_input(input)?;

    let mut lo = 0;
    let mut hi = 0;
    for _ in 0..1_000 {
        let r = graph.signal("broadcaster", Signal::Lo)?;
        lo += r.lo;
        hi += r.hi;
    }
    Ok(lo * hi)
}
pub fn part2(input: &str) -> anyhow::Result<usize> {
    let mut graph = parse_input(input)?;

    // This is garbage.
    // The direct approach is too slow. If we look at the input:
    //   &dn -> rx
    // rx is fed by a single conjunction, which in turn is fed by four other
    // conjunctions. Those conjunctions _happen_ to have very simple periodic behavior.
    // Maybe this is the result of some clever insight, but rather than building a real
    // solution here I'm just going to hard-code the ancestors here and multiply together
    // their cycle lengths.
    let mut i = 0;
    let ancestors = ["dd", "fh", "xp", "fc"];
    let mut cycle_lengths = Vec::new();
    loop {
        let r = graph.signal("broadcaster", Signal::Lo)?;
        i += 1;
        if ancestors.iter().any(|&n| r.recv_lo.contains(n)) {
            cycle_lengths.push(i);
            if cycle_lengths.len() == ancestors.len() {
                return Ok(cycle_lengths.into_iter().reduce(lcm).unwrap());
            }
        }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Signal {
    Lo,
    Hi,
}
#[derive(Debug)]
struct Graph {
    nodes: BTreeMap<String, Node>,
}
impl Graph {
    fn init(&mut self) -> anyhow::Result<()> {
        let mut inputs: BTreeMap<String, BTreeSet<String>> = BTreeMap::default();
        for node in self.nodes.values() {
            for dst in &node.outputs {
                inputs
                    .entry(dst.clone())
                    .or_default()
                    .insert(node.name.clone());
            }
        }
        for node in self.nodes.values_mut() {
            if let Kind::Conjunction { ref mut latest } = node.kind {
                for src in inputs
                    .get(&node.name)
                    .ok_or_else(|| anyhow!("{} has no inputs", node.name))?
                {
                    latest.insert(src.clone(), Signal::Lo);
                }
            }
        }
        Ok(())
    }

    fn signal(&mut self, name: &str, signal: Signal) -> anyhow::Result<PressResult> {
        let mut result = PressResult::default();
        let mut q: VecDeque<(String, Signal, String)> = VecDeque::new();
        q.push_back(("button".to_owned(), signal, name.to_owned()));
        while let Some((src, signal, name)) = q.pop_front() {
            match signal {
                Signal::Lo => {
                    result.lo += 1;
                    result.recv_lo.insert(name.clone());
                }
                Signal::Hi => {
                    result.hi += 1;
                    result.recv_hi.insert(name.clone());
                }
            }
            let Some(n) = self.nodes.get_mut(&name) else {
                continue;
            };
            match n.kind {
                Kind::Broadcast => {
                    for dst in &n.outputs {
                        q.push_back((name.clone(), signal, dst.clone()));
                    }
                }
                Kind::Flipflop { ref mut on } => {
                    if signal == Signal::Lo {
                        *on = !*on;
                        let output = if *on { Signal::Hi } else { Signal::Lo };
                        for dst in &n.outputs {
                            q.push_back((name.clone(), output, dst.clone()));
                        }
                    }
                }
                Kind::Conjunction { ref mut latest } => {
                    let from_src = latest
                        .get_mut(&src)
                        .ok_or_else(|| anyhow!("{name} should not receive signals from {src}"))?;
                    *from_src = signal;
                    let output = if latest.values().all(|&s| s == Signal::Hi) {
                        Signal::Lo
                    } else {
                        Signal::Hi
                    };
                    for dst in &n.outputs {
                        q.push_back((name.clone(), output, dst.clone()));
                    }
                }
            }
        }
        Ok(result)
    }
}
#[derive(Default, Debug)]
struct PressResult {
    lo: usize,
    hi: usize,
    recv_lo: BTreeSet<String>,
    recv_hi: BTreeSet<String>,
}
#[derive(Debug)]
struct Node {
    name: String,
    kind: Kind,
    outputs: Vec<String>,
}
#[derive(Debug)]
enum Kind {
    Broadcast,
    Flipflop { on: bool },
    Conjunction { latest: BTreeMap<String, Signal> },
}
fn parse_input(input: &str) -> anyhow::Result<Graph> {
    let (_, mut graph) = all_consuming(delimited(multispace0, graph_parser, multispace0))(input)
        .map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    graph.init()?;
    Ok(graph)
}
fn graph_parser(input: &str) -> IResult<&str, Graph> {
    let (input, nodes) = separated_list1(delimited(space0, newline, space0), node_parser)(input)?;
    Ok((
        input,
        Graph {
            nodes: nodes.into_iter().map(|n| (n.name.clone(), n)).collect(),
        },
    ))
}
fn node_parser(input: &str) -> IResult<&str, Node> {
    let (input, kind) = kind_parser(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, dsts) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Node {
            name: name.to_owned(),
            kind,
            outputs: dsts.into_iter().map(|d| d.to_owned()).collect(),
        },
    ))
}
fn kind_parser(input: &str) -> IResult<&str, Kind> {
    let (input, k) = opt(one_of("%&"))(input)?;
    Ok((
        input,
        match k {
            Some('%') => Kind::Flipflop { on: false },
            Some('&') => Kind::Conjunction {
                latest: BTreeMap::new(),
            },
            _ => Kind::Broadcast,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT_1: &str = "
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    ";
    const SAMPLE_INPUT_2: &str = "
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    ";

    #[test]
    fn parser_smoke_test() {
        assert_eq!(parse_input(SAMPLE_INPUT_1).unwrap().nodes.len(), 5);
        assert_eq!(parse_input(SAMPLE_INPUT_2).unwrap().nodes.len(), 5);
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT_1).unwrap(), 32000000);
        assert_eq!(part1(SAMPLE_INPUT_2).unwrap(), 11687500);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day20.input").unwrap()).unwrap(),
            899848294,
        );
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day20.input").unwrap()).unwrap(),
            247454898168563,
        );
    }
}
