use std::collections::HashMap;

/*
    Hypothesis, small-small are one way connections, large-small are bidirectional    

    Start at node 'start',
    
    fn dfs(start, visited) -> Option<Path>
        path.insert('start')
        for each node v in start.children
            visited.insert(v);
            if v == end
                return path
            dfs(v, visited)
        return None;

*/
struct Caves {
    graph: HashMap<String, Vec<String>>,
    paths: Vec<Vec<String>>
}

impl Caves {
    fn is_big_cave(cave: &str) -> bool {
        return cave.to_uppercase() == cave;
    }

    fn is_connected(&self, v: &str, w: &str) -> bool {
        return match self.graph.get(v) {
            None => false,
            Some(nodes) => nodes.contains(&w.to_string())
        };
    }

    fn connect(&mut self, v: &str, w: &str) {
        //println!("connect {} to {}", v, w);
        let neighbours = self.graph.entry(v.to_string()).or_insert(Vec::new());
        neighbours.push(w.to_string());
        if !self.is_connected(w, v) {
            self.connect(w, v);
        }
    }

    fn get_neighbours(&self, v: &str) -> Vec<String> {
        return match self.graph.get(v) { Some(n) => n.to_vec(), None => Vec::new() };
    }

    fn depth_first_search<'a>(&mut self, start: &'a str, path: &'a mut Vec<&'a str>) {
        //println!("At Node: {}, path {:?}", start, path);
        path.push(start);
        if start == "end" {
            self.paths.push(path.iter().map(|s| s.to_string()).collect());
            return;
        };

        let neighbours = self.get_neighbours(start);
        neighbours.iter()
            .filter(|v| Caves::is_big_cave(v) || !path.contains(&v.as_str()))
            .for_each(|v| self.depth_first_search(v, &mut path.to_vec()));
            //.flatten()
            //.for_each
            //.collect();
        //return paths;
    }

    fn small_cave_constraint(cave: &str, path: &Vec<&str>) -> bool {
        if !path.contains(&cave) {
            return true;
        } else if "start" == cave {
            return false;
        } else if "end" == cave {
            return false;
        }
        else {
            let mut freq = HashMap::new();
            for small_cave in path.iter().filter(|c| !Caves::is_big_cave(c)) {
                let f = freq.entry(small_cave).or_insert(0);
                *f += 1;
            };
            //println!("{:?}", freq);
            let existingDuplicates = freq.values().filter(|count| *count > &1).count();
            return existingDuplicates == 0;
        }
    }

    fn relaxed_small_cave<'a>(&mut self, start: &'a str, path: &'a mut Vec<&'a str>) {
        //println!("relaxed_small_cave At Node: {}, path {:?}", start, path);
        path.push(start);
        if start == "end" {
            self.paths.push(path.iter().map(|s| s.to_string()).collect());
            return;
        };

        let neighbours = self.get_neighbours(start);
        neighbours.iter()
            .filter(|v| Caves::is_big_cave(v) || Caves::small_cave_constraint(v, path) )
            .for_each(|v| self.relaxed_small_cave(v, &mut path.to_vec()));
            //.flatten()
            //.for_each
            //.collect();
        //return paths;
    }
}

pub fn partOne(input: &str) -> u32 {
    //println!("PArt one input: {}", input);
    let mut caves: Caves = Caves { graph: HashMap::new(), paths: vec![] };
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split("-")
            .collect::<Vec<&str>>())
        .for_each(|nodes| caves.connect(nodes[0], nodes[1]));
    //println!("{:?}", caves.graph);
    caves.depth_first_search("start", &mut Vec::new());
    //caves.paths.iter().for_each(|path| println!("{:?}", path));
    return caves.paths.len() as u32;
}

pub fn partTwo(input: &str) -> u32 {
    //println!("PArt one input: {}", input);
    let mut caves: Caves = Caves { graph: HashMap::new(), paths: vec![] };
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split("-")
            .collect::<Vec<&str>>())
        .for_each(|nodes| caves.connect(nodes[0], nodes[1]));
    //println!("{:?}", caves.graph);
    caves.relaxed_small_cave("start", &mut Vec::new());
    //caves.paths.iter().for_each(|path| println!("{:?}", path));
    return caves.paths.len() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneSmallExample() {
        let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        ";
        let res = partOne(input);
        assert_eq!(10, res);
    }
    
    #[test]
    fn partOneMediumExample() {
        let input = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";
        let res = partOne(input);
        assert_eq!(19, res);
    }
    
    #[test]
    fn partOneBigExample() {
        let input = "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW";
        let res = partOne(input);
        assert_eq!(226, res);
    }

    #[test]
    fn partTwoSmallExample() {
        let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        ";
        let res = partTwo(input);
        assert_eq!(36, res);
    }
    
    #[test]
    fn partTwoMediumExample() {
        let input = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";
        let res = partTwo(input);
        assert_eq!(103, res);
    }
    
    #[test]
    fn partTwoBigExample() {
        let input = "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW";
        let res = partTwo(input);
        assert_eq!(3509, res);
    }
}