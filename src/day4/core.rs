fn is_valid_password(password: &str) -> bool {
    let digits: Vec<u32> = password.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let windows = digits[..].windows(2);
    let increasing = windows.clone().all(|w| w[0] <= w[1]);
    let digit_doubled = windows.clone().any(|w| w[0] == w[1]);

    increasing && digit_doubled
}

fn is_valid_password_2(password: &str) -> bool {
    let digits: Vec<u32> = password.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let windows = digits[..].windows(2);
    let increasing = windows.clone().all(|w| w[0] <= w[1]);
    increasing
        && (0..5).any(|n| {
            digits[n] == digits[n + 1]
                && (n == 0 || (digits[n] != digits[n - 1]))
                && (n == 4 || (digits[n] != digits[n + 2]))
        })
}

fn main() {
    let start = 193651u32;
    let end = 649729u32;
    println!(
        "There are {} valid passwords in the range.",
        (start..end)
            .filter(|p| is_valid_password(&p.to_string()))
            .count()
    );

    println!(
        "There are {} valid passwords in the range (part 2).",
        (start..end)
            .filter(|p| is_valid_password_2(&p.to_string()))
            .count()
    );
}
