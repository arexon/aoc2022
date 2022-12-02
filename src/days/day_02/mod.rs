enum Order {
    Lt,
    Eq,
    Gt,
}

impl From<char> for Order {
    fn from(c: char) -> Self {
        match c {
            'X' => Order::Lt,
            'Y' => Order::Eq,
            'Z' => Order::Gt,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum Attack {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Attack {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Attack::Rock,
            'B' | 'Y' => Attack::Paper,
            'C' | 'Z' => Attack::Scissors,
            _ => unreachable!(),
        }
    }
}

fn score(attack: &Attack) -> u32 {
    match attack {
        Attack::Rock => 1,
        Attack::Paper => 2,
        Attack::Scissors => 3,
    }
}

fn cmp(me: &Attack, opponent: &Attack) -> Order {
    match (&me, &opponent) {
        (Attack::Rock, Attack::Scissors)
        | (Attack::Scissors, Attack::Paper)
        | (Attack::Paper, Attack::Rock) => Order::Gt,
        _ if me == opponent => Order::Eq,
        _ => Order::Lt,
    }
}

fn cmp_score(outcome: Order) -> u32 {
    match outcome {
        Order::Gt => 6,
        Order::Eq => 3,
        Order::Lt => 0,
    }
}

fn to_attack(opponent: &Attack, outcome: &Order) -> Attack {
    match (opponent, outcome) {
        (Attack::Paper, Order::Lt) => Attack::Rock,
        (Attack::Rock, Order::Eq) => Attack::Rock,
        (Attack::Scissors, Order::Gt) => Attack::Rock,
        (Attack::Scissors, Order::Lt) => Attack::Paper,
        (Attack::Paper, Order::Eq) => Attack::Paper,
        (Attack::Rock, Order::Gt) => Attack::Paper,
        (Attack::Rock, Order::Lt) => Attack::Scissors,
        (Attack::Scissors, Order::Eq) => Attack::Scissors,
        (Attack::Paper, Order::Gt) => Attack::Scissors,
    }
}

fn one(input: Vec<&str>) -> u32 {
    let input: Vec<(Attack, Attack)> = input
        .iter()
        .map(|round| {
            let attacks = round.replace(' ', "");
            let mut attacks = attacks.chars();
            (
                Attack::from(attacks.next().unwrap()),
                Attack::from(attacks.next().unwrap()),
            )
        })
        .collect();
    let input: u32 = input
        .iter()
        .map(|(opponent, me)| {
            let attack_score = score(me);
            let outcome = cmp(me, opponent);
            let final_score = cmp_score(outcome);

            attack_score + final_score
        })
        .sum();
    input
}

fn two(input: Vec<&str>) -> u32 {
    let input: Vec<(Attack, Order)> = input
        .iter()
        .map(|round| {
            let attacks = round.replace(' ', "");
            let mut attacks = attacks.chars();
            (
                Attack::from(attacks.next().unwrap()),
                Order::from(attacks.next().unwrap()),
            )
        })
        .collect();
    let input: u32 = input
        .iter()
        .map(|(opponent, outcome)| {
            let me = to_attack(opponent, outcome);
            let attack_score = score(&me);
            let outcome = cmp(&me, opponent);
            let final_score = cmp_score(outcome);

            attack_score + final_score
        })
        .sum();
    input
}

pub fn run(input: &str) -> (String, String) {
    let input: Vec<&str> = input.split('\n').collect();

    (one(input.clone()).to_string(), two(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::Solution;

    #[test]
    fn test_part_one() {
        assert_eq!(
            run(&Solution::new("02").input),
            ("15632".to_string(), "14416".to_string())
        );
    }
}
