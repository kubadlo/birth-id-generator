use chrono::prelude::*;
use rand::prelude::*;

fn is_leap_year(year: &u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn last_day_of_month(year: &u16, month: &u8) -> u8 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

fn generate_year(generator: &mut ThreadRng) -> u16 {
    generator.gen_range(1970..Local::now().year() as u16 + 1)
}

fn generate_month(generator: &mut ThreadRng) -> u8 {
    generator.gen_range(1..13)
}

fn generate_day(generator: &mut ThreadRng, year: &u16, month: &u8) -> u8 {
    generator.gen_range(1..last_day_of_month(year, month) + 1)
}

fn generate_gender(generator: &mut ThreadRng) -> u8 {
    generator.gen_range(0..2)
}

fn generate_seq(generator: &mut ThreadRng) -> u16 {
    generator.gen_range(0..999)
}

fn combine_date_parts(year: &u16, month: &u8, day: &u8, gender: &u8) -> String {
    let year_short: u8 = (year % 100) as u8;
    let month_with_gender: u8 = match *gender {
        1 => *month + 50,
        _ => *month,
    };

    let compare_fmt = |condition_result, value| -> String {
        match condition_result {
            true => format!("0{}", value),
            false => format!("{}", value),
        }
    };

    let year_fmt: String = compare_fmt(year_short < 10, year_short);
    let month_fmt: String = compare_fmt(month_with_gender < 10, month_with_gender);
    let day_fmt: String = compare_fmt(*day < 10, *day);

    format!("{}{}{}", year_fmt, month_fmt, day_fmt)
}

fn create_birth_id(date_part: &String, seq_part: &u16) -> String {
    let date_numeric: u32 = date_part.parse::<u32>().unwrap();

    let control_modulo = (date_numeric + *seq_part as u32) % 11;
    let control_number = match control_modulo {
        10 => 0,
        _ => control_modulo,
    };

    let seq_fmt = match seq_part {
        0..=9 => format!("00{}", seq_part),
        10..=99 => format!("0{}", seq_part),
        _ => format!("{}", seq_part),
    };

    format!("{}/{}{}", date_part, seq_fmt, control_number)
}

fn main() {
    let mut random_generator = rand::thread_rng();

    let year: u16 = generate_year(&mut random_generator);
    let month: u8 = generate_month(&mut random_generator);
    let day: u8 = generate_day(&mut random_generator, &year, &month);
    let gender: u8 = generate_gender(&mut random_generator);
    let seq: u16 = generate_seq(&mut random_generator);

    let date_part: String = combine_date_parts(&year, &month, &day, &gender);
    let birth_id: String = create_birth_id(&date_part, &seq);

    println!("Year:    {}", year);
    println!("Month:   {}", month);
    println!("Day:     {}", day);
    println!(
        "Gender:  {}",
        match gender {
            1 => "Female",
            _ => "Male",
        }
    );

    println!("BirthId: {}", birth_id);
}
