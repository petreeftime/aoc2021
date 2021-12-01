use aoc2021::util::parse_file;

fn day01_part01(depths: &[i32]) -> usize {
    depths
        .windows(2)
        .filter_map(|x| (x[0] < x[1]).then(|| ()))
        .count()
}

fn day01_part02(depths: &[i32]) -> usize {
    let depths_sums = depths
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<_>>();
    day01_part01(&depths_sums)
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "inputs/day01".to_string());
    let depths = parse_file(path).unwrap();
    let res1 = day01_part01(&depths);
    let res2 = day01_part02(&depths);
    println!("{}", res1);
    println!("{}", res2);
}
