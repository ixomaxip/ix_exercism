fn calc(idx: usize, val: u32) -> u32 {
    match idx & 1 {
        1 => {
            let doubled = 2 * val;
            if doubled > 9 {
                doubled - 9
            } else {
                doubled
            }
        }
        _ => val,
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let trimmed = code
        .chars()
        .filter(|c| c.to_string() != " ")
        .collect::<Vec<_>>();

    if trimmed.len() <= 1 {
        return false;
    }

    let (numbers, errors): (Vec<_>, Vec<_>) = trimmed
        .iter()
        .map(|c| c.to_string().parse::<u32>())
        .partition(Result::is_ok);

    if errors.len() != 0 {
        return false;
    }

    let res: u32 = numbers
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| match val {
            Ok(val) => calc(idx, *val),
            Err(_) => unreachable!(),
        })
        .sum();

    res % 10 == 0
}

pub fn is_valid2(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}