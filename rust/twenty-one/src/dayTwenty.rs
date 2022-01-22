
use regex::Regex;
use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
struct Image(Vec<Vec<char>>);

impl FromStr for Image {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let img: Vec<Vec<char>> = input.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().filter(|c| *c == '.' || *c == '#').collect())
            .collect();
        return Ok(Image(img));
    }
}

impl Image {
    fn displayImage(&self) {
        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                print!("{}", self.0[x][y]);
            }
            println!("");
        }
    }

    fn rows(&self) -> usize {
        return self.0.len();
    }

    fn cols(&self) -> usize {
        return self.0[0].len();
    }

    fn neighbours(row: isize, col: isize) -> [(isize, isize); 9] {
        let start_x = match row.checked_sub(1) {
            Some(x) => x,
            None => row
        };
        let start_y = match col.checked_sub(1) {
            Some(x) => x,
            None => col
        };
        let end_x = row + 1;
        let end_y = col + 1;
        let mut points: [(isize, isize); 9] = [(0,0); 9];
        for (i,x) in (start_x..=end_x).enumerate() {
            for (j,y) in (start_y..=end_y).enumerate() {
                points[i*3 + j] = (x,y);
            }
        }
        //println!("neighbours of ({}, {}) = {:?}", row, col, points);
        return points;
    }

    fn window_at(&self, x: isize, y: isize, defaultPixel: char) -> String {
        let window = Image::neighbours(x, y);
        let x = window.iter()
            .map(|(i, j)| self.get(i, j, defaultPixel))
            .map(|c| match c { '#' => '1', _ => '0' })
            .collect();
        //println!("{:?}", x);
        return x;
    }

    fn get(&self, i: &isize, j: &isize, defaultPixel: char) -> char {
        //
        let x: Result<usize, _> = usize::try_from(*i);
        let y: Result<usize, _> = usize::try_from(*j);
        let res = match (x,y) {
            (Ok(x), Ok(y)) => self.0.get(x).and_then(|row| row.get(y)).unwrap_or(&defaultPixel),
            (_, _) => &defaultPixel
        };
        //println!("get value in input image at {}, {} = {}", i, j, res);
        return *res;
    }
}

#[derive(Debug)]
struct ImageEnhancer {
    algorithm: [char; 512],
    image: Image
}

impl FromStr for ImageEnhancer {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut algo: [char; 512] = ['.'; 512];
        input.chars()
            .filter(|c| *c == '.' || *c == '#')
            .take(512)
            .zip(algo.iter_mut())
            .for_each(|(a,b)| *b = a);
        let empty_line = Regex::new(r"\s\n").unwrap();
        let split = empty_line.split(input);
        let img = split.last().unwrap().parse().unwrap();
        
        return  Ok(ImageEnhancer { algorithm: algo, image: img });
    }
}

impl Image {

}

impl ImageEnhancer {
    
    fn enhance(&self, iterations: usize) -> Image {
        let mut img = self.image.clone();
        for i in 0..iterations {
            let output_rows = img.rows() + 2;
            let output_cols = img.cols() + 2;
            let defaultPixel = if self.algorithm[0] == '#' && self.algorithm[self.algorithm.len() - 1] == '.' {
                if i % 2 == 0 { '.' } else { '#' }
                } else { '.' };
            let mut output = Image(vec![vec![defaultPixel; output_cols]; output_rows]);
            //println!("defaultpixel: {}", defaultPixel);
            //println!("output image size {} x {}", output.rows(), output.cols());
            // for each pixel in output img, find the source img pixels
            for x in 0..output_rows {
                for y in 0..output_cols {
                    //println!("output index: ({}, {})", x, y);
                    let index = match usize::from_str_radix(&img.window_at(x as isize - 1, y as isize - 1, defaultPixel), 2) {
                        Ok(nr) => nr,
                        Err(_) => {
                            println!("Error parsing index!");
                            0
                        }
                    };
                    //println!("index: {}", index);
                    output.0[x][y] = self.algorithm[index];
                    //break;
                }
            }
            img = output.clone();
            //img.displayImage();
        }
        return img.clone();
    }
}

pub fn partOne(input: &str) -> u32 {
    let enhancer: ImageEnhancer = input.parse().unwrap();
    //println!("{:?}", enhancer.algorithm);
    //enhancer.image.displayImage();
    let enhancedImage = enhancer.enhance(2);
    
    return enhancedImage.0.iter()
        .map(|row| row.iter().filter(|pixel| **pixel == '#'))
        .flatten()
        .count() as u32;
}


pub fn partTwo(input: &str) -> u32 {
    let enhancer: ImageEnhancer = input.parse().unwrap();
    //println!("{:?}", enhancer.algorithm);
    //enhancer.image.displayImage();
    let enhancedImage = enhancer.enhance(50);
    
    return enhancedImage.0.iter()
        .map(|row| row.iter().filter(|pixel| **pixel == '#'))
        .flatten()
        .count() as u32;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partOneExample() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        
        #..#.
        #....
        ##..#
        ..#..
        ..###
        ";
        let res = partOne(input);
        assert_eq!(35, res);
    }

    #[test]
    fn partTwoExample() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        
        #..#.
        #....
        ##..#
        ..#..
        ..###
        ";
        let res = partTwo(input);
        assert_eq!(3351, res);
    }
}