use std::collections::HashSet;

pub struct Decks {
    p1_deck: Vec<u8>,
    p2_deck: Vec<u8>,
}

#[aoc_generator(day22)]
pub fn parse_input(input: &str) -> Decks {
    let mut players = input.split("\n\n");

    let player1 = players.next().unwrap();
    let mut p1 = player1
        .lines()
        .skip(1)
        .map(|l| l.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    p1.reverse();

    let player2 = players.next().unwrap();
    let mut p2 = player2
        .lines()
        .skip(1)
        .map(|l| l.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    p2.reverse();

    Decks {
        p1_deck: p1,
        p2_deck: p2,
    }
}

fn combat(player1_deck: &mut Vec<u8>, player2_deck: &mut Vec<u8>) -> usize {
    while !player1_deck.is_empty() && !player2_deck.is_empty() {
        let p1 = player1_deck.pop().unwrap();
        let p2 = player2_deck.pop().unwrap();

        if p1 > p2 {
            player1_deck.insert(0, p1);
            player1_deck.insert(0, p2);
        } else {
            player2_deck.insert(0, p2);
            player2_deck.insert(0, p1);
        }
    }

    if !player1_deck.is_empty() {
        calculate_score(&player1_deck)
    } else {
        calculate_score(&player2_deck)
    }
}

#[derive(Debug)]
enum Winner {
    Player1,
    Player2,
}

fn play_recursive_combat(player1_deck: &mut Vec<u8>, player2_deck: &mut Vec<u8>) -> usize {
    match recursive_combat(player1_deck, player2_deck) {
        Winner::Player1 => calculate_score(player1_deck),
        Winner::Player2 => calculate_score(player2_deck),
    }
}

fn recursive_combat(player1_deck: &mut Vec<u8>, player2_deck: &mut Vec<u8>) -> Winner {
    let mut previous_rounds: HashSet<(Vec<u8>, Vec<u8>)> = HashSet::new();
    while !player1_deck.is_empty() && !player2_deck.is_empty() {
        if !previous_rounds.insert((player1_deck.clone(), player2_deck.clone())) {
            return Winner::Player1;
        }

        let p1 = player1_deck.pop().unwrap();
        let p2 = player2_deck.pop().unwrap();

        if p1 as usize <= player1_deck.len() && p2 as usize <= player2_deck.len() {
            let mut new_player1_deck: Vec<u8> = Vec::new();
            for i in &player1_deck[player1_deck.len() - p1 as usize..player1_deck.len()] {
                new_player1_deck.push(*i);
            }

            let mut new_player2_deck: Vec<u8> = Vec::new();
            for i in &player2_deck[player2_deck.len() - p2 as usize..player2_deck.len()] {
                new_player2_deck.push(*i);
            }

            let winner = recursive_combat(&mut new_player1_deck, &mut new_player2_deck);
            match winner {
                Winner::Player1 => {
                    player1_deck.insert(0, p1);
                    player1_deck.insert(0, p2);
                }
                Winner::Player2 => {
                    player2_deck.insert(0, p2);
                    player2_deck.insert(0, p1);
                }
            }
        } else if p1 > p2 {
            player1_deck.insert(0, p1);
            player1_deck.insert(0, p2);
        } else {
            player2_deck.insert(0, p2);
            player2_deck.insert(0, p1);
        }
    }

    if !player1_deck.is_empty() {
        Winner::Player1
    } else {
        Winner::Player2
    }
}

fn calculate_score(deck: &[u8]) -> usize {
    let mut score: usize = 0;
    for (i, u) in deck.iter().enumerate() {
        score += (i + 1) * (*u as usize);
    }
    score
}

#[aoc(day22, part1)]
pub fn part1(decks: &Decks) -> usize {
    let mut p1_deck = decks.p1_deck.to_owned();
    let mut p2_deck = decks.p2_deck.to_owned();

    combat(&mut p1_deck, &mut p2_deck)
}

#[aoc(day22, part2)]
pub fn part2(decks: &Decks) -> usize {
    let mut p1_deck = decks.p1_deck.to_owned();
    let mut p2_deck = decks.p2_deck.to_owned();

    play_recursive_combat(&mut p1_deck, &mut p2_deck)
}
