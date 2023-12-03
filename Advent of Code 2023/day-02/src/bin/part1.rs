

fn main() {
    let input = include_str!("./input1.txt");

    let output: u32 = input
    .lines()
    .map(process1)
    .sum::<u32>();

    dbg!(output);
}

fn process1(game: &str) -> u32 {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes?
    let red_max: u32 = 12;
    let green_max: u32 = 13;
    let blue_max: u32 = 14;
    

    // return true if any is invalid
    let is_not_valid_game =game.split(&[':', ';',','][..]) // returns an iterator for each bit in the line
    .any(|bit_of_text|{

        let number: u32 = bit_of_text.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap();
         
        if bit_of_text.contains("red") && number > red_max {
            true
        } else if bit_of_text.contains("blue") && number > blue_max {
            true
        } else if bit_of_text.contains("green") && number > green_max {
            true
        } else {
            false
        }
    });

    if is_not_valid_game {
        0
    } else {
        game.split(&[':']).next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".lines().map(process1).sum::<u32>();
        assert_eq!(result,8);
    }

    #[test]
    fn invalid_red() {
        let result = "Game 1: 13 red, 2 blue, 4 green".lines().map(process1).sum::<u32>();
        assert_eq!(result,0);
    }

    #[test]
    fn invalid_green() {
        let result = "Game 1: 0 red, 2 blue, 14 green".lines().map(process1).sum::<u32>();
        assert_eq!(result,0);
    }
    
    #[test]
    fn invalid_blue() {
        let result = "Game 1: 0 red, 15 blue, 10 green".lines().map(process1).sum::<u32>();
        assert_eq!(result,0);
    }
}