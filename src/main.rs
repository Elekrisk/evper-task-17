#![feature(iter_intersperse)]
use std::io::{BufRead, stdin};

fn main() {
    let mut word_list: Vec<Vec<char>> = Vec::with_capacity(5_000_000);
    let mut misspelled: Vec<Vec<char>> = Vec::with_capacity(100);

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    // FOR DEBUGGING PURPOSES
    #[cfg(feature = "testing")]
    let input_file = std::env::args().skip(1).next().expect("path to test file expected");
    #[cfg(feature = "testing")]
    let input = std::fs::read_to_string(input_file).unwrap();
    #[cfg(feature = "testing")]
    let mut lines = input.lines().map(|l| Result::<_, ()>::Ok(l.to_string()));

    for line in &mut lines {
        let line = line.unwrap();
        if line.bytes().next().unwrap() == b'#' {
            break;
        }

        word_list.push(line.chars().collect());
    }

    misspelled.extend(lines.map(|l| l.unwrap().chars().collect()));


    let mut matrix = [[0; 41]; 41];
    for i in 1..41 {
        matrix[i][0] = i as u8;
        matrix[0][i] = i as u8;
    }

    let mut last = &vec![];
    
    for word1 in &misspelled {
        let l1 = word1.len();
        
        let mut min_dist = u8::MAX;
        let mut res = vec![];

        for word2 in &word_list {
            let l2 = word2.len();
            if if l1 > l2 { l1 - l2 } else { l2 - l1 } > min_dist as usize { continue; } 

            let mut start = 1;
            for i in 0..last.len().min(l2) {
                if last[i] != word2[i] {
                    break;
                }
                start += 1;
            }

            #[cfg(feature = "testing")]
            eprintln!("   {}", word2.iter().map(|c| vec![' ', ' ', *c]).flatten().collect::<String>());

            for p1 in 1..=word1.len() {
                #[cfg(feature = "testing")]
                eprint!("  {}", word1[p1 - 1]);
                for p2 in start..=word2.len() {
                    let a = matrix[p1 - 1][p2] + 1;
                    let b = matrix[p1][p2 - 1] + 1;
                    let c = matrix[p1 - 1][p2 - 1] + if word1[p1 - 1] == word2[p2 - 1] { 0 } else { 1 };
                    let d = a.min(b).min(c);
                    matrix[p1][p2] = d;
                    #[cfg(feature = "testing")]
                    eprint!("{:3}", d);
                }
            #[cfg(feature = "testing")]
            eprintln!();
            }

            let d = matrix[l1 as usize][l2 as usize];
            if d < min_dist {
                min_dist = d;
                res.clear();
                res.push(word2);
            } else if d == min_dist {
                res.push(word2);
            }
            last = word2;
        }
        println!("{} ({}) {}", word1.iter().collect::<String>(), min_dist, res.iter().map(|cc| cc.iter().collect::<String>()).intersperse(" ".into()).collect::<String>())
    }
}
