use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    // build Dictionary from rules
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut iter = input.lines();
    iter.by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            if let Some((a, b)) = line.split_once('|') {
                let k = a.parse::<u32>().unwrap();
                let v = b.parse::<u32>().unwrap();
                match map.contains_key(&k) {
                    false => {
                        map.insert(k, vec![v]);
                    }
                    true => {
                        map.get_mut(&k).unwrap().push(v);
                    }
                }
            }
        });

    let result: u32 = iter
        .filter_map(|line| {
            let pages: Vec<u32> = line
                .split(',')
                .map(|value| value.parse::<u32>().unwrap())
                .collect();

            for i in 1..pages.len() {
                if let Some(should_be_after) = map.get(&pages[i]) {
                    if should_be_after.iter().any(|num| pages[0..i].contains(num)) {
                        return None;
                    }
                }
            }
            Some(pages[pages.len() / 2])
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, "143".to_string())
    }
}
