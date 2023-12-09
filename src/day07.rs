/*
Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an
airship. (At least it's a cool airship!) It drops you off at the edge of a vast
desert and descends back to Island Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing, wearing
goggles, and riding a large camel.

"Did you bring the parts?" she asks again, louder this time. You aren't sure
what parts she's looking for; you're here to figure out why the sand stopped.

"The parts! For the sand, yes! Come with me; I will show you." She beckons you
onto the camel.

After riding a bit across the sands of Desert Island, you can see what look like
very large rocks covering half of the horizon. The Elf explains that the rocks
are all along the part of Desert Island that is directly above Island Island,
making it hard to even get there. Normally, they use big machines to move the
rocks and filter the sand, but the machines have broken down because Desert
Island recently stopped receiving the parts they need to fix the machines.

You've already assumed it'll be your job to figure out why the parts stopped
when she asks if you can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the game of
Camel Cards. Camel Cards is sort of similar to poker except it's designed to be
easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order them based on
the strength of each hand. A hand consists of five cards labeled one of A, K, Q,
J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this
order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

- Five of a kind, where all five cards have the same label: AAAAA
- Four of a kind, where four cards have the same label and one card has a different label: AA8AA
- Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
- Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
- Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
- One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
- High card, where all cards' labels are distinct: 23456

Hands are primarily ordered based on type; for example, every full house is
stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect. Start by
comparing the first card in each hand. If these cards are different, the hand
with the stronger first card is considered stronger. If the first card in each
hand have the same label, however, then move on to considering the second card
in each hand. If they differ, the hand with the higher second card wins;
otherwise, continue with the third card in each hand, then the fourth, then the
fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because
its first card is stronger. Similarly, 77888 and 77788 are both a full house,
but 77888 is stronger because its third card is stronger (and both hands have
the same first and second card).

To play Camel Cards, you are given a list of hands and their corresponding bid
(your puzzle input). For example:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

This example shows five hands; each hand is followed by its bid amount. Each
hand wins an amount equal to its bid multiplied by its rank, where the weakest
hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the
strongest hand. Because there are five hands in this example, the strongest hand
will have rank 5 and its bid will be multiplied by 5.

So, the first step is to put the hands in order of strength:

- 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
- KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
- T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.

Now, you can determine the total winnings of this set of hands by adding up the
result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3
+ 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

Find the rank of every hand in your set. What are the total winnings?
*/

/*
To make things a little more interesting, the Elf introduces one additional
rule. Now, J cards are jokers - wildcards that can act like whatever card would
make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even than
2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2,
J.

J cards can pretend to be whatever card is best for the purpose of determining
hand type; for example, QJJQ2 is now considered four of a kind. However, for the
purpose of breaking ties between two hands of the same type, J is always treated
as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J
is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
KK677 is now the only two pair, making it the second-weakest hand.
T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
*/

use std::collections::BTreeMap;

use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{all_consuming, map_res, value},
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let mut bids = parse_bids(input)?;
    bids.sort_by_cached_key(|b| (categorize(b.hand), b.hand));
    let total = bids
        .into_iter()
        .enumerate()
        .map(|(i, bid)| (i + 1) as u64 * bid.amount)
        .sum::<u64>();
    Ok(total)
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let bids = parse_bids(input)?;
    let mut categorized: Vec<(Kind, Bid)> = bids
        .into_iter()
        .map(|bid| {
            // Translate J into jokers, the lowest value card.
            let hand: [u8; 5] = bid
                .hand
                .into_iter()
                .map(|card| if card == 11 { 1 } else { card })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let mut t = tally(hand);

            // It is always optimal to coerce the joker into the modal non-joker card.
            let mut candidates: Vec<u8> = t.keys().filter(|&&k| k != 1).copied().collect();
            candidates.sort_by_cached_key(|k| t.get(k).copied().unwrap());
            // It is technically possible that there is no candidate because the hand is entirely jokers
            if let Some(best) = candidates.last() {
                if let Some(jokers) = t.remove(&1) {
                    *t.get_mut(best).unwrap() += jokers;
                }
            }
            (
                compute_kind(t),
                Bid {
                    hand,
                    amount: bid.amount,
                },
            )
        })
        .collect();
    categorized.sort_by_key(|&(kind, bid)| (kind, bid.hand));
    let total = categorized
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid.amount)
        .sum::<u64>();
    Ok(total)
}

#[derive(Debug, Copy, Clone)]
struct Bid {
    hand: [u8; 5],
    amount: u64,
}

fn parse_bids(input: &str) -> anyhow::Result<Vec<Bid>> {
    let (_, bids) = all_consuming(delimited(multispace0, bids_parser, multispace0))(input)
        .map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    Ok(bids)
}

fn bids_parser(input: &str) -> IResult<&str, Vec<Bid>> {
    separated_list1(multispace1, bid_parser)(input)
}
fn bid_parser(input: &str) -> IResult<&str, Bid> {
    let (input, (hand, amount)) = separated_pair(hand_parser, multispace1, num_parser)(input)?;
    Ok((input, Bid { hand, amount }))
}
fn hand_parser(input: &str) -> IResult<&str, [u8; 5]> {
    let (input, cards) = count(card_parser, 5)(input)?;
    Ok((input, cards.try_into().unwrap()))
}
fn card_parser(input: &str) -> IResult<&str, u8> {
    alt((
        value(14, tag("A")),
        value(13, tag("K")),
        value(12, tag("Q")),
        value(11, tag("J")),
        value(10, tag("T")),
        value(9, tag("9")),
        value(8, tag("8")),
        value(7, tag("7")),
        value(6, tag("6")),
        value(5, tag("5")),
        value(4, tag("4")),
        value(3, tag("3")),
        value(2, tag("2")),
    ))(input)
}
fn num_parser(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Triple,
    FullHouse,
    Quad,
    Quint,
}

fn categorize(hand: [u8; 5]) -> Kind {
    compute_kind(tally(hand))
}
fn tally(hand: [u8; 5]) -> BTreeMap<u8, usize> {
    let mut tally = BTreeMap::new();
    for card in hand {
        *tally.entry(card).or_default() += 1;
    }
    tally
}
fn compute_kind(t: BTreeMap<u8, usize>) -> Kind {
    let mut counts: Vec<usize> = t.values().copied().collect();
    counts.sort();
    if counts == vec![5] {
        return Kind::Quint;
    }
    if counts == vec![1, 4] {
        return Kind::Quad;
    }
    if counts == vec![2, 3] {
        return Kind::FullHouse;
    }
    if counts == vec![1, 1, 3] {
        return Kind::Triple;
    }
    if counts == vec![1, 2, 2] {
        return Kind::TwoPair;
    }
    if counts == vec![1, 1, 1, 2] {
        return Kind::OnePair;
    }
    Kind::HighCard
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn categorize_test() {
        assert_eq!(categorize([1, 1, 1, 1, 1]), Kind::Quint);
        assert_eq!(categorize([1, 1, 2, 1, 1]), Kind::Quad);
        assert_eq!(categorize([1, 1, 2, 1, 2]), Kind::FullHouse);
        assert_eq!(categorize([1, 1, 2, 1, 3]), Kind::Triple);
        assert_eq!(categorize([1, 1, 2, 2, 3]), Kind::TwoPair);
        assert_eq!(categorize([1, 1, 2, 3, 4]), Kind::OnePair);
        assert_eq!(categorize([1, 2, 3, 4, 5]), Kind::HighCard);
    }

    const SAMPLE_INPUT: &str = "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    ";

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 6440);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day07.input").unwrap()).unwrap(),
            248105065,
        );
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT).unwrap(), 5905);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day07.input").unwrap()).unwrap(),
            249515436,
        );
    }
}
