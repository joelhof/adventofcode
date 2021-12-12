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
    graph: HashMap<String, Vec<String>>
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
        let neighbours = self.graph.entry(v.to_string()).or_insert(Vec::new());
        neighbours.push(w.to_string());
        if Caves::is_big_cave(v) && !self.is_connected(v, w) {
            self.connect(w, v);
        }
    }

    
}

pub fn partOne(input: &str) -> u32 {
    let mut caves: Caves = Caves { graph: HashMap::new() };
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split("-")
            .collect::<Vec<&str>>())
        .for_each(|nodes| caves.connect(nodes[0], nodes[1]));
    println!("{:?}", caves.graph);
    return 0;
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
}