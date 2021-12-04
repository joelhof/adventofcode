use std::collections::HashMap;

#[derive(Debug)]
struct BingoBoard {
    numbers: HashMap<u32, (u32,u32)>,
    columnMarks: HashMap<u32, u32>,
    rowMarks: HashMap<u32, u32>
}

impl BingoBoard {
    fn mark(&mut self, nr: &u32) {
        match self.numbers.get(nr) {
            None => (),
            Some((row, col)) => {
                self.columnMarks.insert(
                    *col,
                    match self.columnMarks.get(col) {
                        None => 0,
                        Some(x) => *x
                    } + 1
                );
                self.rowMarks.insert(
                    *row,
                    match self.rowMarks.get(row) {
                        None => 0,
                        Some(x) => *x
                    } + 1
                );
            }
        }
        self.numbers.remove(nr);
    }

    fn isWinner(&self) -> bool {
        let winnerRows: Vec<(&u32, &u32)> = self.rowMarks.iter()
            .filter(|(_k, v)| **v >= 5 )
            .collect();
        winnerRows.iter().for_each(|(k,_v)| println!("Bingo! on row: {}", k));
        let winnerColumns: Vec<(&u32, &u32)> = self.columnMarks.iter()
            .filter(|(_k, v)| **v >= 5 )
            .collect();
        winnerColumns.iter().for_each(|(k,_v)| println!("Bingo! on column: {}", k));
        let isWinner = winnerRows.len() > 0 || winnerColumns.len() > 0;
        if isWinner {
            println!("winning board! {:?}", self)
        }
        return isWinner;
    }

    fn sumUnmarked(&self) -> u32 {
        return self.numbers.keys()
            .sum();
    }
}

fn parseBoards(input: &str) -> Vec<BingoBoard> {
    let mut boardLines = input.lines().skip(1);
    let mut currentRow = 0;
    let mut currentBoardIndex = 0;
    let mut boards: Vec<BingoBoard> = Vec::new();
    while let Some(line) = boardLines.next() {
//        println!("----");
//        println!("{}", line);
        if line.trim().is_empty() {
            boards.push(BingoBoard { 
                numbers: HashMap::new(),
                columnMarks: HashMap::new(),
                rowMarks: HashMap::new()
            });
            currentRow = 0;
            currentBoardIndex = boards.len() - 1;
        } else {
//            println!("{}", line.trim());
            for (i, nr) in line.trim().split_whitespace().enumerate() {
//                println!("{} {}", i, nr);
                boards[currentBoardIndex].numbers.insert(nr.parse().unwrap(), (currentRow, i as u32));
            }
//            println!("curr row {} : {:?}", currentRow, boards[currentBoardIndex].numbers);
            currentRow = currentRow + 1;
        }
    }
    return boards;
}

pub fn partOne(input: &str) -> u32 {

    let drawOrder: Vec<u32> = input.lines().next().unwrap()
        .split(",")
        .map(|nr| nr.parse().unwrap())
        .collect();

    
    let mut boards: Vec<BingoBoard> = parseBoards(input);
    
//    println!("{:?}", boards);

    for draw in drawOrder.iter() {
        for i in 0..boards.len() {
            boards[i].mark(draw);
        }

        let winners: Vec<&BingoBoard> = boards.iter().filter(|b| b.isWinner()).collect();
        if winners.len() > 0 {
            return winners[0].sumUnmarked() * draw;
        }
    }

    return 0;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
        
        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7";
        let res = partOne(input);
        assert_eq!(4512, res);
    }
}