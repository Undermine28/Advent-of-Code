fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> i32 {

    let mut sum = 0;
    for line in input.lines() {
        let digit1_index = line.find(|c | char::is_digit(c, 10)).unwrap();
        let digit2_index = line.rfind(|c| char::is_digit(c,10)).unwrap();

        let digit1 = &line.chars().nth(digit1_index).unwrap();
        let digit2 = &line.chars().nth(digit2_index).unwrap();
        let mut fakestring: String = Default::default();
        fakestring.push(*digit1);
        fakestring.push(*digit2);
        sum += fakestring.parse::<i32>().unwrap();
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result,142);
    }
}