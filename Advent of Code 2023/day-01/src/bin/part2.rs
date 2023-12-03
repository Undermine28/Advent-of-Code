use std::{process::Output, f32::INFINITY};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

// let line2 = line.replace("zero", "0");
// let line3 = line2.replace("one", "1");
// let line4 = line3.replace("two", "2");
// let line5 = line4.replace("three","3");
// let line6 = line5.replace("four","4");
// let line7 = line6.replace("five", "5");
// let line8 = line7.replace("six", "6");
// let line9 = line8.replace("seven","7");
// let line10 = line9.replace("eight","8");
// line10.replace("nine", "9")

fn part2(input: &str) -> u32 {

    let lst_of_words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


    let output = input
     .lines()
     .map(|line|{

        let mut min_index = usize::MAX;
        let mut replace_number = 10;
        for number in 0..10 {
            match line.find(lst_of_words[number]) {
                Some(new_index) => {
                    if new_index <min_index{
                        min_index = new_index;
                        replace_number = number;
                    }
                },
                None => {},
            }
        }

        let mut line2: String = line.to_owned();
        match replace_number {
            10 => line2 = line.to_owned(),
            _ => match char::from_digit(replace_number.try_into().unwrap(),10){
                        Some(number) =>line2.insert(min_index+2, number),
                        _ => panic!()
                    }
        }

        // if replace_number_end != 10 {
        //     match char::from_digit(replace_number_end,10){
        //         Some(number) =>line2.insert(max_index+2, number),
        //         _ => panic!()
        //     }
        //     // line2.replace(lst_of_words[replace_number_end],&replace_number_end.to_string())
        // } 


        // Now we go backwards
        let mut max_index = usize::MIN;
        let mut replace_number_end = 10;
        for number in 0..10 {
            match line.rfind(lst_of_words[number]) {
                Some(new_index) => {
                    if new_index >max_index {
                        max_index = new_index;
                        replace_number_end = number;
                    }
                },
                None => {},
            }
        }
        if replace_number_end != 10 {
            match char::from_digit(replace_number_end.try_into().unwrap(),10){
                Some(number) =>line2.insert(max_index+2, number),
                _ => panic!()
            }
            // line2.replace(lst_of_words[replace_number_end],&replace_number_end.to_string())
        } 
        line2

     }).inspect(|v|{
        dbg!(v);
     })
     .map(|line| { 
        let mut it =
        line.chars().filter_map(|character| {
            character.to_digit(10)
        });

        let first = 
        it.next().expect("should be a number");

        match it.last() {
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}"),
        }.parse::<u32>().expect("should be valid")

     })
     .sum::<u32>();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part2("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result,142);
    }

    #[test]
    fn test2() {
        let result = part2("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result,281);
    }
    #[test]
    fn test3() {
        let result = part2("sevenine
        oneight
        ");
        assert_eq!(result,97);
    }
}

// 22
// 75
// 54
// 83
// 99