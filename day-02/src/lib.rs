pub fn check_report(levels: &[i32]) -> bool {
    match levels[0].cmp(&levels[1]) {
        std::cmp::Ordering::Less => levels.windows(2).all(|window| {
            let diff = window[1] - window[0];
            0 < diff && diff < 4
        }),
        std::cmp::Ordering::Greater => levels.windows(2).all(|window| {
            let diff = window[0] - window[1];
            0 < diff && diff < 4
        }),
        std::cmp::Ordering::Equal => false,
    }
}
