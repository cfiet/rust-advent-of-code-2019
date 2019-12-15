fn check_number_part1(v: i32) -> bool {
    let num = v.to_string();
    let s = num.as_bytes();
    let mut consecutive = false;

    for i in 1usize..s.len() {
        if s[i - 1] > s[i] {
            return false;
        }
        if s[i - 1] == s[i] {
            consecutive = true
        };
    }

    consecutive
}

fn check_number_part2(v: i32) -> bool {
    let num = v.to_string();
    let s = num.as_bytes();
    let mut consecutive = false;
    let mut repeted_num = 0;
    let mut repeted_count = 0;

    for i in 1usize..s.len() {
        if s[i - 1] > s[i] {
            return false;
        }
        if s[i - 1] == s[i] {
            repeted_num = s[i];
            repeted_count += 1;
        } else {
            if repeted_count == 1 && repeted_num == s[i - 1] {
                consecutive = true
            }
            repeted_num = 0;
            repeted_count = 0;
        }
    }

    consecutive || repeted_count == 1
}

fn main() {
    let c1 = (272_091..815_432)
        .filter(|v| check_number_part1(*v))
        .count();

    println!("Matching: {}", c1);

    let c2 = (272_091..815_432)
        .filter(|v| check_number_part2(*v))
        .count();

    println!("Matching: {}", c2);
}
