fn main() {
    let input = include_str!("./input1.txt");

    let output: u32 = input
    .lines()
    .map(process2)
    .sum::<u32>();

    dbg!(output);
}

fn process2(game: &str) -> u32 {
    // initialize all at 0
    let mut red_max = 0;
    let mut green_max = 0;
    let mut blue_max = 0;
    
    // return true if any is invalid
    game.split(&[':', ';',','][..]) // returns an iterator for each bit in the line
    .for_each(|bit_of_text|{

        let number: u32 = bit_of_text.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap();
         
        if bit_of_text.contains("red") && number > red_max {
            red_max = number
        } else if bit_of_text.contains("blue") && number > blue_max {
            blue_max = number
        } else if bit_of_text.contains("green") && number > green_max {
            green_max = number
        }
    });

    red_max*green_max*blue_max
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
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".lines().map(process2).sum::<u32>();
        assert_eq!(result,2286);
    }

    #[test]
    fn game1power() {
        let result = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".lines().map(process2).sum::<u32>();
        assert_eq!(result,48);
    }

    #[test]
    fn game2power() {
        let result = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".lines().map(process2).sum::<u32>();
        assert_eq!(result,12);
    }

    #[test]
    fn game3power() {
        let result = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".lines().map(process2).sum::<u32>();
        assert_eq!(result,1560);
    }

    #[test]
    fn game4power() {
        let result = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".lines().map(process2).sum::<u32>();
        assert_eq!(result,630);
    }

    #[test]
    fn game5power() {
        let result = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".lines().map(process2).sum::<u32>();
        assert_eq!(result,36);
    }
}