use std::fs::read_to_string;
use itertools::Itertools;

fn read_lines_advent1(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut l: Vec<i32> = vec![];
    let mut r: Vec<i32> = vec![];

    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .for_each(|s| {
            let mut split = s.split("   ");

            let ls = split.next().unwrap();
            let rs = split.next().unwrap();

            l.push(ls.parse().unwrap());
            r.push(rs.parse().unwrap());
        }) ;

    (l, r)
}

pub fn advent1a() {
    let (mut l, mut r) = read_lines_advent1("advent1-input.txt");

    l.sort();
    r.sort();

    let diff : i32 = l.into_iter()
        .zip(r.into_iter())
        .map(|(li, ri)| {
            (li - ri).abs()
        })
        .sum();

    println!("{}", diff);
}

#[test]
pub fn advent1b() {
    let (mut l, mut r) = read_lines_advent1("advent1-input.txt");

    l.sort();
    r.sort();

    let r_count = r.into_iter()
                .map(|i| (i, i))
                .into_group_map();

    let similarity : i32 = l.into_iter()
        .map(|li| {
            let rv = r_count.get(&li);

            let mult = if let Some(rv) = rv {
                rv.len() as i32
            } else {
                0
            };

            li * mult
        })
        .sum();

    println!("{}", similarity);
}