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
        return cave.to_uppercase() == cave || cave == "start";
    }

    fn is_connected(&self, v: &str, w: &str) -> bool {
        return match self.graph.get(v) {
            None => false,
            Some(nodes) => nodes.contains(&w.to_string())
        };
    }

    fn connect(&mut self, v: &str, w: &str) {
        let neighbours = self.graph.entry(v.to_string()).or_insert(Vec::new());
        neighbours.push(w.to_string());
        if (Caves::is_big_cave(v) || Caves::is_big_cave(w)) && !self.is_connected(w, v) {
            self.connect(w, v);
        }
    }

    fn get_neighbours(&self, v: &str) -> Vec<String> {
        return match self.graph.get(v) { Some(n) => n.to_vec(), None => Vec::new() };
    }

    fn depth_first_search<'a>(&mut self, start: &'a str, path: &'a mut Vec<&'a str>) {
        println!("At Node: {}, path {:?}", start, path);
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
}

pub fn partOne(input: &str) -> u32 {
    let mut caves: Caves = Caves { graph: HashMap::new(), paths: vec![] };
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split("-")
            .collect::<Vec<&str>>())
        .for_each(|nodes| caves.connect(nodes[0], nodes[1]));
    println!("{:?}", caves.graph);
    caves.depth_first_search("start", &mut Vec::new());
    caves.paths.iter().for_each(|path| println!("{:?}", path));
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
}