use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Player {
    id: u8,
    postion: CylindricalPosition,
    points: u32
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CylindricalPosition(u8);

impl CylindricalPosition {
    fn add(&self, other: u32) -> Self {
        //println!("adding {} + {}", self.0, other);
        let v = self.0 as u32 + other;
        if v < 10 {
            return CylindricalPosition(v as u8);
        }
        let overflow = v % 10;
        
        if overflow != 0 {
            return CylindricalPosition(overflow as u8);
        }
        //println!("Number is > 10 and divisible by 10 {} {} / 10 = {}", overflow, v, v / 10);
        return CylindricalPosition((10) as u8);
    }
}

impl FromStr for Player {
    type Err = &'static str;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let caps = Regex::new(r"Player (\d+) starting position: (\d+)").unwrap().captures(input).unwrap();
        let id = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
        let pos = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        return Ok(Player { id: id, postion: CylindricalPosition(pos), points: 0 });
    }
}

#[derive(Debug)]
struct DiracDiceEngine {
    players: Vec<Player>,
    rolls: u32 
}

impl FromStr for DiracDiceEngine {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut players: Vec<Player> = vec![];
        for l in input.lines() {
            let p: Player = l.parse().unwrap();
            players.push(p);
        }
        return Ok(DiracDiceEngine { players: players, rolls: 0 });
    }
}

impl DiracDiceEngine {
    fn roll(&mut self, times: u32) {
        // for current player, roll dice times nr of times
        // add score to position
        // move player to position
        let isPlayerOne = ((self.rolls / times) % 2) == 0;
        let mut currentPlayer = self.players.get_mut(if isPlayerOne { 0 } else { 1 } ).unwrap();
        //println!("current pos {}", currentPlayer.start_postion.0);
        self.rolls += times;
        let mut sum = 0;
        for _i in 0..times {
            sum += self.rolls - 1;
        }
        currentPlayer.postion = currentPlayer.postion.add(sum as u32);
        currentPlayer.points += currentPlayer.postion.0 as u32;
        //self.players[ if isPlayerOne { 0 } else { 1 } ] = currentPlayer;
        //println!("roll count {} player id {} next pos {} points {}", self.rolls, currentPlayer.id, currentPlayer.start_postion.0, currentPlayer.points);
    }

    fn rolls(&self) -> u32 {
        return self.rolls;
    }

    fn loser(&self) -> Option<&Player> {
        if self.players[0].points >= 1000 {
            return Some(&self.players[1]);
        }
        if self.players[1].points >= 1000 {
            return Some(&self.players[0]);
        }
        return None;
    }
}

pub fn partOne(input: &str) -> u32 {
    let mut engine: DiracDiceEngine = input.parse().unwrap();
    
    while let None = engine.loser() {
        engine.roll(3);
    }
    return engine.loser().unwrap().points * engine.rolls();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parseTest() {
        let input = "Player 1 starting position: 4
        Player 2 starting position: 8";
        let track: DiracDiceEngine = input.parse().unwrap();
        assert_eq!(track.players[0], "Player 1 starting position: 4".parse().ok().unwrap());
    }

    #[test]
    fn addWithOverflowTest() {
        let res = CylindricalPosition(6).add(88+89+90);
        assert_eq!(3, res.0);

        let res = CylindricalPosition(4).add(91+92+93);
        assert_eq!(10, res.0);
    }

    #[test]
    fn partOneExample() {
        let input = "Player 1 starting position: 4
        Player 2 starting position: 8";
        let res = partOne(input);
        assert_eq!(739785, res);
    }
}