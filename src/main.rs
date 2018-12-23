extern crate chrono;
extern crate rand;

use chrono::prelude::*;
use rand::prelude::*;

pub fn is_leap_year(year: &u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn last_day_of_month(year: &u16, month: &u8) -> u8 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) {
            29
        } else {
            28
        },
        _ => 31,
    }
}

fn generate_year(generator: &mut ThreadRng) -> u16 {
    generator.gen_range(1970, Local::now().year() as u16 + 1)
}

fn generate_month(generator: &mut ThreadRng) -> u8 {
    generator.gen_range(1, 13)
}

fn generate_day(generator: &mut ThreadRng, year: &u16, month: &u8) -> u8 {
    generator.gen_range(1, last_day_of_month(year, month) + 1)
}

fn generate_gender(generator: &mut ThreadRng) -> u8 {
    generator.gen_range(0, 2)
}

fn main() {
    let mut random_generator = rand::thread_rng();

    let year: u16 = generate_year(&mut random_generator);
    let month: u8 = generate_month(&mut random_generator);
    let day: u8 = generate_day(&mut random_generator, &year, &month);
    let gender: u8 = generate_gender(&mut random_generator);

    println!("Day: {}", day);
    println!("Month: {}", month);
    println!("Year: {}", year);
    println!("Gender: {}", gender);
}
