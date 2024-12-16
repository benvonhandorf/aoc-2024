use itertools::{iproduct, Itertools};
use regex::Regex;
use std::{
    collections::HashMap, fs::read_to_string, io::Error, ops::{Index, IndexMut}, slice::RChunks
};

fn read_order_lines(filename: &str) -> Vec<(usize, usize)> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|s| {
            let mut parts = s.split("|");
            (parts.next().unwrap().parse::<usize>().unwrap(), parts.next().unwrap().parse::<usize>().unwrap())
        })
        .collect()
}

fn read_update_lines(filename: &str) -> Vec<Vec<usize>> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|s| {
            let mut parts = s.split(",");

            parts.map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}



fn update_is_in_order(update: &Vec<usize>, rules:&HashMap<usize, Vec<usize>>) -> bool {
    for pivot in 1..update.len() {
        let current = update[ pivot ];
        let prior = &update[0..pivot];

        let rules_for_current = rules.get(&current);
        
        if let Some(rules_for_current) = rules_for_current {
            for rule in rules_for_current {
                if prior.contains(rule) {
                    println!("Update failed due to {rule} preceeding {current} in position {pivot}");
                    return false
                }
            }
        }
    }

    return true;
}

fn load_data_for_token(token: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let order_items = read_order_lines(format!("advent5a-{}-order.txt", token).as_str());

    let updates = read_update_lines(format!("advent5a-{}-update.txt", token).as_str());

    (order_items, updates)

}

fn updates_in_order(token: &str) -> Vec<usize> {
    let order_items = read_order_lines(format!("advent5a-{}-order.txt", token).as_str());

    let order_rules = order_items.into_iter()
            .into_group_map();


    let updates = read_update_lines(format!("advent5a-{}-update.txt", token).as_str());

    updates.iter().enumerate()
        .filter(|(_idx, update)| update_is_in_order(update, &order_rules))
        .map(|(idx, _update)| idx)
        .collect()
}

#[test]
fn find_correct_rule_sum() {
    let (order_items, updates) = load_data_for_token("input");

    let order_rules = order_items.into_iter()
            .into_group_map();

    let passing_rules: Vec<usize> = updates.iter().enumerate()
            .filter(|(_idx, update)| update_is_in_order(update, &order_rules))
            .map(|(idx, _update)| idx)
            .collect();

    println!("Passing rules: {}", passing_rules.len());

    dbg!(&passing_rules);

    let middle_page_sum : u32 = passing_rules.into_iter()
        .map(|idx| &updates[idx])
        .map(|v| v.get( (v.len() / 2) ).unwrap() )
        .map(|v| *v as u32 )
        .sum();

    assert_eq!(143, middle_page_sum);
}

fn fix_broken_rules(input_update: &Vec<usize>, rules:&HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut update = input_update.clone();

    for pivot in 1..update.len() {
        let current = update[ pivot ];
        let prior = &update[0..pivot];

        let rules_for_current = rules.get(&current);
        
        if let Some(rules_for_current) = rules_for_current {
            for rule in rules_for_current {
                let violator = prior.into_iter().enumerate()
                                    .filter(|(_, x)| *x == rule)
                                    .next();

                if let Some((idx, _)) = violator {
                    update.swap(pivot, idx);

                    return fix_broken_rules(&update, rules);
                }
            }
        }
    }

    update
}

#[test]
fn fix_broken_rule_update_4() {
    let order_items: Vec<(usize, usize)> = vec![(97, 75)];

    let order_rules = order_items.into_iter()
            .into_group_map();

    let update: Vec<usize> = vec![75,97,47,61,53];

    assert_eq!(vec![97,75,47,61,53], fix_broken_rules(&update, &order_rules));
}

#[test]
fn fix_broken_rule_update_6() {
    let order_items: Vec<(usize, usize)> = vec![(47,53),
    (97,13),
    (97,61),
    (97,47),
    (75,29),
    (61,13),
    (75,53),
    (29,13),
    (97,29),
    (53,29),
    (61,53),
    (97,53),
    (61,29),
    (47,13),
    (75,47),
    (97,75),
    (47,61),
    (75,61),
    (47,29),
    (75,13),
    (53,13),];

    let order_rules = order_items.into_iter()
            .into_group_map();

    let update: Vec<usize> = vec![97,13,75,29,47];

    assert_eq!(vec![97,75,47,29,13], fix_broken_rules(&update, &order_rules));
}

#[test]
fn find_broken_rule_sum_input() {
    let (order_items, updates) = load_data_for_token("input");

    let order_rules = order_items.into_iter()
            .into_group_map();

    let fixed_rules : Vec<Vec<usize>> = updates.iter().enumerate()
            .filter(|(_idx, update)| !update_is_in_order(update, &order_rules))
            .map(|(_, update)| fix_broken_rules(update, &order_rules))
            .collect();

    println!("Broken rules: {}", fixed_rules.len());

    dbg!(&fixed_rules);

    let middle_page_sum : u32 = fixed_rules.into_iter()
        .map(|v| {
            let l = &v.len();

            v.get( (l / 2) ).unwrap().clone()
        } )
        .map(|v| v as u32 )
        .sum();

    assert_eq!(123, middle_page_sum);
}


#[test]
fn find_broken_rule_sum_test() {
    let (order_items, updates) = load_data_for_token("test");

    let order_rules = order_items.into_iter()
            .into_group_map();

    let fixed_rules : Vec<Vec<usize>> = updates.iter().enumerate()
            .filter(|(_idx, update)| !update_is_in_order(update, &order_rules))
            .map(|(_, update)| fix_broken_rules(update, &order_rules))
            .collect();

    println!("Broken rules: {}", fixed_rules.len());

    dbg!(&fixed_rules);

    let middle_page_sum : u32 = fixed_rules.into_iter()
        .map(|v| {
            let l = &v.len();

            v.get( (l / 2) ).unwrap().clone()
        } )
        .map(|v| v as u32 )
        .sum();

    assert_eq!(123, middle_page_sum);
}


#[test]
fn test_update_4() {
    let order_items: Vec<(usize, usize)> = vec![(97, 75)];

    let order_rules = order_items.into_iter()
            .into_group_map();

    let update: Vec<usize> = vec![75,97,47,61,53];

    assert_eq!(false, update_is_in_order(&update, &order_rules));
}

#[test]
fn test_update_5() {
    let order_items: Vec<(usize, usize)> = vec![(29, 13)];

    let order_rules = order_items.into_iter()
            .into_group_map();

    let update: Vec<usize> = vec![61, 13, 29];

    assert_eq!(false, update_is_in_order(&update, &order_rules));
}

#[test]
fn test_updates_in_order() {
    let in_order = updates_in_order("test");

    dbg!(&in_order);

    assert_eq!(3, in_order.len());
}

#[test]
fn build_order_lookups() {
    let order_rules = read_order_lines("advent5a-test-order.txt");

    let ordered_map = order_rules.into_iter()
        .into_group_map();

    dbg!(ordered_map);
}

#[test]
fn load_order_lines() {
    let order_rules = read_order_lines("advent5a-test-order.txt");

    assert_eq!(21, order_rules.len());

    dbg!(&order_rules);
}

#[test]
fn load_update_lines() {
    let updates = read_update_lines("advent5a-test-update.txt");

    dbg!(&updates);

    assert_eq!(6, updates.len());
}