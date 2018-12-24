use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

const INPUT: &str = include_str!("../input");
const VERBOSE: bool = false;

lazy_static! {
    static ref GROUP_REGEX: Regex = Regex::new(concat!(
        r"(?P<units>\d+) units.*with (?P<hitpoints>\d+) hit points.*",
        r"does (?P<attack_damage>\d+) (?P<attack_type>\w+) damage.*initiative (?P<initiative>\d+)"
    ))
    .unwrap();
    static ref IMMUNITIES_REGEX: Regex = Regex::new(r"immune to ((?:\w+(?:, )?)+)").unwrap();
    static ref WEAKNESSES_REGEX: Regex = Regex::new(r"weak to ((?:\w+(?:, )?)+)").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AttackType {
    Radiation,
    Bludgeoning,
    Fire,
    Slashing,
    Cold,
}

impl AttackType {
    fn from_str(s: &str) -> Self {
        use crate::AttackType::*;
        match s {
            "radiation" => Radiation,
            "bludgeoning" => Bludgeoning,
            "fire" => Fire,
            "slashing" => Slashing,
            "cold" => Cold,
            _ => panic!("Invalid attack type: {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Team {
    ImmuneSystem,
    Infection,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Team::ImmuneSystem => "Immune System",
            Team::Infection => "Infection",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
struct Group {
    id: usize,
    team: Team,
    units: RefCell<u32>,
    hit_points: u32,
    attack_damage: u32,
    attack_type: AttackType,
    initiative: u32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
    target: RefCell<Weak<Group>>,
}

impl Group {
    fn from_input_line(line: &str, id: usize, team: Team) -> Self {
        let captures = GROUP_REGEX.captures(line).unwrap();
        let units = RefCell::new(captures.name("units").unwrap().as_str().parse().unwrap());
        let hit_points = captures
            .name("hitpoints")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let attack_damage = captures
            .name("attack_damage")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let attack_type = AttackType::from_str(captures.name("attack_type").unwrap().as_str());
        let initiative = captures
            .name("initiative")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let immunities = match IMMUNITIES_REGEX.captures(line) {
            Some(caps) => caps[1].split(", ").map(AttackType::from_str).collect(),
            None => vec![],
        };
        let weaknesses = match WEAKNESSES_REGEX.captures(line) {
            Some(caps) => caps[1].split(", ").map(AttackType::from_str).collect(),
            None => vec![],
        };
        Self {
            id,
            team,
            units,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
            target: RefCell::new(Weak::new()),
        }
    }

    fn effective_power(&self) -> u32 {
        *self.units.borrow() * self.attack_damage
    }

    fn calculate_attack_damage_to(&self, other: &Self) -> u32 {
        if other.immunities.contains(&self.attack_type) {
            return 0;
        }
        let mut damage = self.effective_power();
        if other.weaknesses.contains(&self.attack_type) {
            damage *= 2;
        }
        damage
    }

    fn key(&self) -> (usize, Team) {
        (self.id, self.team)
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} group {}", self.team, self.id)
    }
}

impl Hash for Group {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key().hash(state);
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for Group {}

fn parse_groups(input: &str) -> Vec<Group> {
    let input = input.trim();
    let parts: Vec<_> = input.split("\n\n").collect();
    let mut result = vec![];
    result.extend(
        parts[0]
            .lines()
            .skip(1)
            .enumerate()
            .map(|(i, line)| Group::from_input_line(line, i + 1, Team::ImmuneSystem)),
    );
    result.extend(
        parts[1]
            .lines()
            .skip(1)
            .enumerate()
            .map(|(i, line)| Group::from_input_line(line, i + 1, Team::Infection)),
    );
    result
}

fn simulate_combat(groups: &mut Vec<Rc<Group>>) {
    let mut last_unit_count = count_units(&groups);
    while find_winning_team(&groups).is_none() {
        fight(groups);
        let unit_count = count_units(&groups);
        if unit_count == last_unit_count {
            if VERBOSE {
                println!("Stalemate");
            }
            break;
        }
        last_unit_count = unit_count;
    }
}

fn fight(groups: &mut Vec<Rc<Group>>) {
    if VERBOSE {
        println!("Immune System:");
        for group in groups.iter().filter(|g| g.team == Team::ImmuneSystem) {
            println!("Group {} contains {} units", group.id, group.units.borrow());
        }
        println!("Infection:");
        for group in groups.iter().filter(|g| g.team == Team::Infection) {
            println!("Group {} contains {} units", group.id, group.units.borrow());
        }
        println!();
    }
    select_targets(groups);
    if VERBOSE {
        println!();
    }
    resolve_attacks(groups);
    if VERBOSE {
        println!();
    }
}

fn select_targets(groups: &mut Vec<Rc<Group>>) {
    groups.sort_by_key(|g| Reverse((g.effective_power(), g.initiative)));
    select_targets_for_team(groups, Team::Infection);
    select_targets_for_team(groups, Team::ImmuneSystem);

    fn select_targets_for_team(groups: &mut Vec<Rc<Group>>, team: Team) {
        let mut reserved_targets = HashSet::new();
        for attacker in groups.iter().filter(|g| g.team == team) {
            let target = groups
                .iter()
                .filter(|g| attacker.team != g.team && !reserved_targets.contains(g))
                .max_by_key(|g| {
                    let potential_damage = attacker.calculate_attack_damage_to(g);
                    if VERBOSE {
                        println!(
                            "{} group {} would deal defending group {} {} damage",
                            attacker.team, attacker.id, g.id, potential_damage
                        );
                    }
                    (potential_damage, g.effective_power(), g.initiative)
                });
            if target.is_none() || attacker.calculate_attack_damage_to(target.unwrap()) == 0 {
                *attacker.target.borrow_mut() = Weak::new();
                continue;
            }
            let target = target.unwrap();
            reserved_targets.insert(target);
            *attacker.target.borrow_mut() = Rc::downgrade(target);
        }
    }
}

fn resolve_attacks(groups: &mut Vec<Rc<Group>>) {
    groups.sort_by_key(|g| Reverse(g.initiative));
    for attacker in groups.iter() {
        let target = attacker.target.borrow().upgrade();
        if target.is_none() || *attacker.units.borrow() == 0 {
            continue;
        }
        let target = target.unwrap();
        let damage = attacker.calculate_attack_damage_to(&target);
        let killed_units = u32::min(damage / target.hit_points, *target.units.borrow());
        *target.units.borrow_mut() -= killed_units;
        if VERBOSE {
            println!(
                "{} group {} attacks defending group {}, killing {} units",
                attacker.team, attacker.id, target.id, killed_units
            );
        }
    }
    groups.retain(|g| *g.units.borrow() > 0);
}

fn find_winning_team(groups: &[Rc<Group>]) -> Option<Team> {
    if !groups.iter().all(|g| g.team == groups[0].team) {
        None
    } else {
        Some(groups[0].team)
    }
}

fn count_units(groups: &[Rc<Group>]) -> usize {
    groups.iter().map(|g| *g.units.borrow() as usize).sum()
}

fn simulate_with_immune_boost(groups: &[Group], immune_boost: usize) -> (Option<Team>, usize) {
    let mut groups = groups.to_vec();
    for immune_group in groups.iter_mut().filter(|g| g.team == Team::ImmuneSystem) {
        immune_group.attack_damage += immune_boost as u32;
    }
    let mut groups: Vec<_> = groups.iter().cloned().map(Rc::new).collect();
    simulate_combat(&mut groups);
    let winning_team = find_winning_team(&groups);
    let unit_count = count_units(&groups);
    (winning_team, unit_count)
}

fn solve_part_one(groups: &[Group]) -> usize {
    let mut groups: Vec<_> = groups.iter().cloned().map(Rc::new).collect();
    simulate_combat(&mut groups);
    count_units(&groups)
}

fn solve_part_two(groups: &[Group]) -> usize {
    let mut lower_limit = 0;
    let mut upper_limit = 0;
    loop {
        let (winning_team, _) = simulate_with_immune_boost(groups, upper_limit);
        if winning_team.is_some() && winning_team.unwrap() == Team::ImmuneSystem {
            break;
        }
        upper_limit = if upper_limit == 0 { 1 } else { upper_limit * 2 };
    }
    while lower_limit <= upper_limit {
        let boost = (lower_limit + upper_limit) / 2;
        let (winning_team, _) = simulate_with_immune_boost(groups, boost);
        if winning_team.is_some() && winning_team.unwrap() == Team::ImmuneSystem {
            upper_limit = boost - 1;
        } else {
            lower_limit = boost + 1;
        }
    }
    let (_, unit_count) = simulate_with_immune_boost(groups, lower_limit);
    if VERBOSE {
        println!("Boost of {}", lower_limit);
    }
    unit_count
}

fn main() {
    let groups = parse_groups(INPUT);
    println!("{}", solve_part_one(&groups));
    println!("{}", solve_part_two(&groups));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input");
    const PART_ONE_SAMPLE_SOLUTION: usize = 5216;
    const PART_TWO_SAMPLE_SOLUTION: usize = 51;

    const PART_ONE_SOLUTION: usize = 15470;
    const PART_TWO_SOLUTION: usize = 5742;

    #[test]
    fn it_solves_part_one_sample_correctly() {
        let groups = parse_groups(SAMPLE_INPUT);
        assert_eq!(solve_part_one(&groups), PART_ONE_SAMPLE_SOLUTION);
    }
    #[test]
    fn it_solves_part_two_sample_correctly() {
        let groups = parse_groups(SAMPLE_INPUT);
        assert_eq!(solve_part_two(&groups), PART_TWO_SAMPLE_SOLUTION);
    }

    #[test]
    fn it_solves_part_one_correctly() {
        let groups = parse_groups(INPUT);
        assert_eq!(solve_part_one(&groups), PART_ONE_SOLUTION);
    }
    #[test]
    fn it_solves_part_two_correctly() {
        let groups = parse_groups(INPUT);
        assert_eq!(solve_part_two(&groups), PART_TWO_SOLUTION);
    }
}
