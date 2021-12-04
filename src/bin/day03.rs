use aoc2021::util::parse_file;

fn day03_part01(bin_numbers: &[String]) -> i64 {
    let bits = bin_numbers.iter().map(|b| b.trim().len()).max().unwrap();
    let bits_min = bin_numbers.iter().map(|b| b.trim().len()).min().unwrap();
    assert_eq!(bits_min, bits);

    let numbers = bin_numbers
        .iter()
        .map(|n| i64::from_str_radix(n.trim(), 2))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut gamma = 0;
    let mut epsilon = 0;
    for b in 0..bits_min {
        let mask = 1 << b;
        let c = numbers.iter().fold((0, 0), |c, n| {
            if n & mask == 0 {
                (c.0, c.1 + 1)
            } else {
                (c.0 + 1, c.1)
            }
        });
        if c.1 > c.0 {
            gamma = gamma | mask;
        } else {
            epsilon = epsilon | mask;
        }
    }

    gamma * epsilon
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/day03".to_string());
    let bin_numbers: Vec<String> = parse_file(path).unwrap();
    let p1 = day03_part01(&bin_numbers);
    println!("{}", p1);
}
