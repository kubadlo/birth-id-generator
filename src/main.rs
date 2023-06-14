use chrono::prelude::*;
use rand::prelude::*;

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

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
    generator.gen_range(1954..=Local::now().year() as u16)
}

fn generate_month(generator: &mut ThreadRng) -> u8 {
    generator.gen_range(1..=12)
}

fn generate_day(generator: &mut ThreadRng, year: &u16, month: &u8) -> u8 {
    generator.gen_range(1..=last_day_of_month(year, month))
}

fn generate_gender(generator: &mut ThreadRng) -> Gender {
    match generator.gen_range(0..=1) {
        1 => Gender::Female,
        _ => Gender::Male,
    }
}

fn generate_seq(generator: &mut ThreadRng) -> u16 {
    generator.gen_range(0..=999)
}

fn combine_date_parts(year: &u16, month: &u8, day: &u8, gender: &Gender) -> String {
    let year_short: u8 = (year % 100) as u8;
    let month_with_gender: u8 = match *gender {
        Gender::Female => *month + 50,
        _ => *month,
    };

    format!("{:0>2}{:0>2}{:0>2}", year_short, month_with_gender, *day)
}

fn create_birth_id(date_part: &String, seq_part: &u16) -> String {
    let date_numeric: u32 = date_part.parse::<u32>().unwrap();

    let control_modulo = (date_numeric + *seq_part as u32) % 11;
    let control_number = match control_modulo {
        10 => 0,
        _ => control_modulo,
    };

    format!("{}/{:0>3}{}", date_part, seq_part, control_number)
}

fn main() {
    let mut random_generator = rand::thread_rng();

    let year: u16 = generate_year(&mut random_generator);
    let month: u8 = generate_month(&mut random_generator);
    let day: u8 = generate_day(&mut random_generator, &year, &month);
    let gender: Gender = generate_gender(&mut random_generator);
    let seq: u16 = generate_seq(&mut random_generator);

    let date_part: String = combine_date_parts(&year, &month, &day, &gender);
    let birth_id: String = create_birth_id(&date_part, &seq);

    println!("Year:    {}", year);
    println!("Month:   {}", month);
    println!("Day:     {}", day);
    println!("Gender:  {:?}", gender);
    println!("BirthId: {}", birth_id);
}
