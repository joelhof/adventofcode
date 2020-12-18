#![allow(non_snake_case)]

use crate::core::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day {
    input: String
}

struct TicketInfo {
    metadata: HashMap<String, Vec<(u64, u64)>>, // a list of ranges
    myTicket: Vec<u64>,
    nearbyTickets: Vec<Vec<u64>>
}

impl FromStr for TicketInfo {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut inputIter = input.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty());
        let mut metadata: HashMap<String, Vec<(u64, u64)>> = HashMap::new();
        let mut ticketMetadata = true;
        while ticketMetadata {
            let line = inputIter.next();
            ticketMetadata = match line {
                None => false,
                Some(l) => {
                    !l.starts_with("your ticket")
                }
            };
            if ticketMetadata {
                let ranges = parseRanges(line.unwrap());
                let key = line.unwrap().split(":").nth(0).unwrap();
                metadata.insert(key.to_string(), ranges);
            }
        }

        let myTicket: Vec<u64> = parseTicket(inputIter.next().unwrap());
        
        inputIter.next();
        let nearby: Vec<Vec<u64>> = inputIter
            .map(|line| parseTicket(line))
            .collect();
        return Ok(TicketInfo{
            metadata: metadata,
            myTicket: myTicket,
            nearbyTickets: nearby
         }); 
    }
}

impl TicketInfo {
    fn isNumberValid(&self, nr: &u64) -> bool {
        return self.metadata.values().into_iter()
                    .flatten()
                    .any(|(min, max)| {
                        //println!("min {} max {}", min, max);
                        nr <= max && nr >= min
                    })
    }
}

impl Day {
    fn init(input: &str) -> Day {
        return Day {
            input: input.to_string()
        }
    }

    pub fn new() -> Day {
        return Day::init(&loadInput("Sixteen"));
    }
}

impl AdventOfCodeSolver for Day {
    fn day(&self) -> &str {
        return "Sixteen";
    }

    fn partOne(&self) -> u64 {
        let mut inputIter = self.input.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty());
        let mut ticketRanges: Vec<Vec<(u64, u64)>> = Vec::new();
        let mut ticketMetadata = true;
        while ticketMetadata {
            let line = inputIter.next();
            ticketMetadata = match line {
                None => false,
                Some(l) => {
                    !l.starts_with("your ticket")
                }
            };
            if ticketMetadata {
                let ranges = parseRanges(line.unwrap());
                ticketRanges.push(ranges);
            }
        }
        //println!("{:?}", ticketRanges);

        let myTicket: Vec<u64> = parseTicket(inputIter.next().unwrap());
        //println!("ticket {:?}", myTicket);
        inputIter.next();
        let errorRate: u64 = inputIter
            .map(|ticket| parseTicket(ticket))
            .flatten()
            .filter(|nr| {
                //println!("{}", nr);
                let res = !ticketRanges[..].into_iter()
                        .flatten()
                        .any(|(min, max)| {
                            //println!("min {} max {}", min, max);
                            nr <= max && nr >= min
                        });
                //println!("{}", res);
                res
            }
            ).sum();
        return errorRate;
    }

    fn partTwo(&self) -> u64 {
        let ticketInfo: TicketInfo = self.input.parse().unwrap();
        let nrOfFields = ticketInfo.myTicket.len();
        let mut mapPostionToMetadata: HashMap<usize, String> = HashMap::new();

        let numbers: Vec<&u64> = ticketInfo.nearbyTickets[..].into_iter()
            .filter(|ticket| ticket.into_iter()
                .all(|nr| ticketInfo.isNumberValid(nr))
            ).flatten().collect();
        println!("{:?}", numbers);

        let result = assign(0, &numbers, nrOfFields, &ticketInfo.metadata);
        println!("{:?}", result);
        let product: u64 = result.unwrap().iter()
            .filter(|(_idx, field)| field.starts_with("departure"))
            .map(|(idx, _f)| ticketInfo.myTicket[*idx])
            .product();
        return product;
        /* for pos in 0..nrOfFields {
            let numbers: Vec<&u64> = ticketInfo.nearbyTickets[pos..].into_iter()
                .filter(|ticket| ticket.into_iter()
                    .all(|nr| ticketInfo.isNumberValid(nr)
                    )
                )
                .flatten().collect();
            //println!("pos {} {:?}", pos, numbers);
            let matchingMetadata: Vec<_> = (&ticketInfo.metadata).into_iter()
                .filter(|(_key, ranges)| 
                    {   //println!("{}: {:?}", _key, ranges);
                        numbers[..].into_iter().step_by(nrOfFields)
                            .all(|field| {
                                let res = ranges[..].into_iter()
                                .any(|(min, max)| {
                                    //println!("min {} max {}", min, max);
                                    *field <= max && *field >= min
                                });
                                //println!("{} res {}", field, res);
                                res
                            })
                        }
                ).map(|(key, _r)| key).collect();
            //println!("{:?}", matchingMetadata);
            mapPostionToMetadata.insert(pos, matchingMetadata);
        }
        println!("{:?}", mapPostionToMetadata);
        //println!("{:?}", ticketInfo.metadata);
        let mut posMap: Vec<_> = mapPostionToMetadata.into_iter().collect();
        posMap.sort_by(|(_k1,v1),(_k2,v2)| v1.len().cmp(&v2.len()));
        
        let mut finalMap: HashMap<usize, String> = depthFirstSearch(&posMap);
        //println!("{:?}", finalMap);
        let sum: u64 = finalMap.iter()
            .filter(|(_idx, field)| field.starts_with("departure"))
            .map(|(idx, _f)| ticketInfo.myTicket[*idx])
            .sum();
        //mapFieldToKey.iter().filter(|(key, pos)| key.starts_with(""))
        return sum; */
    }
}

fn assign(col: usize, columns: &Vec<&u64>, rowSize: usize, labels: &HashMap<String, Vec<(u64, u64)>>) -> Option<Vec<(usize, String)>> {
    //println!("Recur {:?} {:?} {:?}", col, columns, labels);
    for (label, constraints) in labels {
        // check if label is valid for column col
        let isLabelValid = columns[col..].into_iter().step_by(rowSize)
            .all(|field| {
                            let res = constraints[..].into_iter()
                                    .any(|(min, max)| {
                                    //println!("min {} max {}", min, max);
                                    *field <= max && *field >= min
                                });
                                //println!("{} res {}", field, res);
                                res
                            });
        //println!("{}, {} {}", isLabelValid, label, col);
        if isLabelValid {
            let mut availableLables = labels.clone();
            availableLables.remove(label);
            let mut candidate = vec![(col, label.to_string())];
            if availableLables.is_empty() {
                return Some(candidate);
            }
            let res = assign(col + 1, columns, rowSize, &availableLables);
            //println!("result {:?}", res);
            if res.is_some() {
                candidate.extend_from_slice(&res.unwrap()[..]);
                //println!("Exiting loop...{:?}", candidate);
                return Some(candidate);
            }
        }
    }
    return None;
}

fn depthFirstSearch(posMap: &Vec<(usize, Vec<&String>)>) -> HashMap<usize, String> {
    let mut taken: HashSet<String> = HashSet::new();
    let mut finalMap: HashMap<usize, String> = HashMap::new();
    for (k, v) in posMap {
        let first = v.iter().cloned().find(|m| !taken.contains(*m));

        finalMap.insert(*k, first.unwrap().to_string());
        taken.insert(first.unwrap().to_string());
    }
    return finalMap;
}

fn parseTicket(row: &str) -> Vec<u64> {
    return row.split(",")
        .map(|nr| nr.parse().unwrap())
        .collect();
}

fn parseRanges(line: &str) -> Vec<(u64, u64)> {
    let rangePattern = Regex::new(r"([0-9]*-[0-9]*)").unwrap();
    //println!("line {:?}", line);
    let r = line.split(":").nth(1).unwrap();
    let range = rangePattern.captures_iter(r)
        .into_iter()
    //.for_each(|cap| println!("cap {:?}", cap));
        .map(|cap| cap.get(1).map_or("", |r| r.as_str()))
        .map(|rStr| rStr.split("-")
            .map(|r| r.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
        )
        .map(|v| (v[0], v[1]))
        .collect::<Vec<(u64, u64)>>();
    //println!("{:?}", range);
    return range;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sixteenPartOneExampleTest() {
        const INPUT: &str = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50
        
        your ticket:
        7,1,14
        
        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";
        let result = Day::init(INPUT).partOne();
        assert_eq!(result, 71);
    }

    #[test]
    fn sixteenPartTwoExampleTest() {
        const INPUT: &str = "class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19
        
        your ticket:
        11,12,13
        
        nearby tickets:
        3,9,18
        15,1,5
        5,14,9";
        let result = Day::init(INPUT).partTwo();
        assert_eq!(result, 13);
    }

    #[test]
    fn sixteenPartTwoExampleTest2() {
        const INPUT: &str = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50
        
        your ticket:
        7,1,14
        
        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";
        let result = Day::init(INPUT).partTwo();
        assert_eq!(result, 13);
    }
}