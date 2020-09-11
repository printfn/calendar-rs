type Year = u64;
type Month = usize;
type DayOfWeek = u64;

const fn is_leap_year(year: Year) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

const fn num_days_in_year(year: Year) -> u64 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

const fn num_days_in_month(year: Year, month: Month) -> u64 {
    if month == 2 {
        if is_leap_year(year) {
            29
        } else {
            28
        }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else {
        31
    }
}

const fn day_of_week_of_first_of_month(mut year: Year, mut month: Month) -> DayOfWeek {
    let mut num_days = 70_000_000; // number of days shifted forward
    while month > 1 {
        month -= 1;
        num_days += num_days_in_month(year, month);
    }
    while year < 2016 {
        num_days -= num_days_in_year(year);
        year += 1;
    }
    while year > 2016 {
        year -= 1;
        num_days += num_days_in_year(year);
    }
    let day_of_week = 5; // Friday
    (day_of_week + num_days + 6) % 7 + 1
}

/// print the specified month, or print whitespace if passed 0
fn print_month_row_space_2(month: Month) {
    let months = [
        "   ", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    print!("{}", months[month]);
}

fn print_month_row(year: Year, month: Month, row: i64) {
    let column_of_first = day_of_week_of_first_of_month(year, month);
    let mut day = 1 + 7 * row - column_of_first as i64;
    for i in 0..7 {
        if row == 0 && i == 0 {
            print_month_row_space_2(month);
        } else if row == 0 && i == 1 {
            if day == 1 {
                print!("1 ");
            }
            day += 1;
            continue;
        }
        if day >= 1 && day <= num_days_in_month(year, month) as i64 {
            print!("{:>2}", day);
        } else if row == 0 && day == 0 && i == 0 {
            // do nothing
        } else {
            print!("  ");
        }
        if i != 6 {
            print!(" ");
        }
        day += 1;
    }
}

const fn bottom_row_empty(year: Year, month: Month) -> bool {
    let column_of_first = day_of_week_of_first_of_month(year, month);
    let day = 1 + 7 * 5 - column_of_first;
    day > num_days_in_month(year, month)
}

const fn bottom_row_is_completely_empty(year: Year, mr: usize) -> bool {
    bottom_row_empty(year, 0 * 4 + mr + 1)
        && bottom_row_empty(year, 1 * 4 + mr + 1)
        && bottom_row_empty(year, 2 * 4 + mr + 1)
}

fn print_calendar(year: Year) {
    println!(
        "Su Mo Tu We Th Fr Sa  Su Mo   {:^4}   Fr Sa  Su Mo Tu We Th Fr Sa",
        year
    );
    for row in 0..24 {
        if row % 6 == 5 && bottom_row_is_completely_empty(year, row / 6) {
            continue; // skip sixth row
        }
        for mc in 0..3 {
            let month = mc * 4 + row / 6 + 1;
            if mc > 0 {
                print!("  ");
            }
            print_month_row(year, month, row as i64 % 6);
        }
        println!();
    }
}

fn get_current_year() -> Result<Year, ()> {
    use std::time::SystemTime;
    let mut year = 1970;
    let mut seconds_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| ())?
        .as_secs();
    while seconds_since_epoch >= num_days_in_year(year) * 86400 {
        seconds_since_epoch -= num_days_in_year(year) * 86400;
        year += 1;
    }
    Ok(year)
}

fn main() {
    if let Some(year_str) = std::env::args().nth(1) {
        if let Ok(selected_year) = year_str.parse::<Year>() {
            if selected_year >= 1 && selected_year <= 9999 {
                print_calendar(selected_year)
            } else {
                eprintln!("Please specify a valid year between 1 and 9999")
            }
        } else {
            eprintln!("Please specify a valid year between 1 and 9999")
        }
    } else if let Ok(current_year) = get_current_year() {
        print_calendar(current_year)
    } else {
        eprintln!("Unable to get current time")
    }
}
