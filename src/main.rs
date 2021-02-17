use std::{hint::unreachable_unchecked, io::{Read, stdin}, ops::{Index, IndexMut}};

#[derive(Copy, Clone)]
struct SVecC {
    inner: [u8; 40],
    len: usize
}

impl SVecC {
    pub fn new() -> Self {
        Self {
            inner: [0; 40],
            len: 0
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn chars<'a>(&'a self) -> SVecCIter<'a> {
        SVecCIter {
            inner: &self.inner[0..self.len],
            index: 0
        }
    }

    #[inline]
    pub fn push(&mut self, c: u8) {
        self.inner[self.len] = c;
        self.len += 1;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl Index<usize> for SVecC {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for SVecC {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

struct SVecCIter<'a> {
    inner: &'a [u8],
    index: usize
}

impl<'a> Iterator for SVecCIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.len() {
            None
        } else {
            let ret = match self.inner[self.index] {
                0xa5 => 'å',
                0xa4 => 'ä',
                0xb6 => 'ö',
                o => o as char
            };
            self.index += 1;
            Some(ret)
        }
    }
}

fn main() {
    #[cfg(feature = "bench")]
    let tot_start = std::time::Instant::now();
    
    #[cfg(feature = "bench")]
    let alloc_start = std::time::Instant::now();
    let mut word_list: Vec<SVecC> = Vec::with_capacity(500_000);
    let mut misspelled: Vec<SVecC> = Vec::with_capacity(100);
    #[cfg(feature = "bench")]
    let alloc_end = std::time::Instant::now();
    
    let mut stdin = std::io::stdin();
    #[cfg(not(feature = "testing"))]
    let mut input = Vec::with_capacity(201_000_000);
    #[cfg(feature = "bench")]
    let in_start = std::time::Instant::now();
    #[cfg(not(feature = "testing"))]
    match stdin.read_to_end(&mut input) { Ok(_) => {}, Err(_) => unsafe { unreachable_unchecked() } };
    #[cfg(feature = "bench")]
    let in_end = std::time::Instant::now();
    #[cfg(not(feature = "testing"))]
    let mut bytes = input.into_iter(); //input.bytes();
    
    // FOR DEBUGGING PURPOSES
    #[cfg(feature = "testing")]
    let input_file = std::env::args().skip(1).next().expect("path to test file expected");
    #[cfg(feature = "testing")]
    let input = std::fs::read(input_file).unwrap();
    #[cfg(feature = "testing")]
    let mut bytes = input.into_iter();
    
    #[cfg(feature = "bench")]
    let convert_start = std::time::Instant::now();

    let mut buffer = SVecC::new();
    loop {
        if let Some(c) = bytes.next() {
            match c {
                b'#' => break,
                b'\n' => {
                    word_list.push(buffer);
                    buffer.clear();
                },
                b'\r' => {},
                0xc3 => buffer.push(match bytes.next() { Some(v) => v, None => unsafe { unreachable_unchecked() } }),
                o => buffer.push(o)
            }
        } else {
            break
        }
    }
    if bytes.next().unwrap() == b'\r' {
        bytes.next();
    }
    
    loop {
        if let Some(c) = bytes.next() {
            match c {
                b'\n' => {
                    misspelled.push(buffer);
                    buffer.clear();
                },
                b'\r' => {},
                0xc3 => buffer.push(match bytes.next() { Some(v) => v, None => unsafe { unreachable_unchecked() } }),
                o => buffer.push(o)
            }
        } else {
            break;
        }
    }
    #[cfg(feature = "bench")]
    let convert_end = std::time::Instant::now();
    #[cfg(feature = "bench")]
    let algo_start = std::time::Instant::now();

    
    let mut matrix = [[0; 41]; 41];
    for i in 1..41 {
        matrix[i][0] = i as u8;
        matrix[0][i] = i as u8;
    }

    let mut results = vec![];
    
    
    for word1 in &misspelled {
        let l1 = word1.len();
        
        let mut min_dist = std::u8::MAX;
        let mut res = vec![];
        
        let mut last = &SVecC::new();
        
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

            for p1 in 1..=word1.len() {

                for p2 in start..=word2.len() {
                    #[cfg(feature = "testing")]
                    {
                        eprint!("      ");
                        for c in word2.chars() {
                            eprint!("{:>3}", c);
                        }
                        eprintln!();
                        for (p11, c) in std::iter::once((0, ' ')).chain(word1.chars().enumerate().map(|(i, c)| (i + 1, c))) {
                            eprint!("{:>3}", c);
                            for p22 in 0..l2 + 1 {
                                if p11 == p1 && p22 == p2 {
                                    eprint!("{:>3}", format!("!{}", matrix[p11][p22]));
                                } else {
                                    eprint!("{:3}", matrix[p11][p22]);
                                }
                            }
                            eprintln!();
                        }
                        eprintln!();
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer).unwrap();
                    }
                    let a = matrix[p1 - 1][p2] + 1;
                    let b = matrix[p1][p2 - 1] + 1;
                    let c = matrix[p1 - 1][p2 - 1] + if word1[p1 - 1] == word2[p2 - 1] { 0 } else { 1 };
                    let d = a.min(b).min(c);
                    matrix[p1][p2] = d;
                }
            }
            #[cfg(feature = "testing")]
            {
                eprint!("      ");
                for c in word2.chars() {
                    eprint!("{:>3}", c);
                }
                eprintln!();
                for (p1, c) in std::iter::once((0, ' ')).chain(word1.chars().enumerate().map(|(i, c)| (i + 1, c))) {
                    eprint!("{:>3}", c);
                    for p2 in 0..l2 + 1 {
                        eprint!("{:3}", matrix[p1][p2]);
                    }
                    eprintln!();
                }
                eprintln!();
                let mut buffer = String::new();
                stdin.read_line(&mut buffer).unwrap();
            }

            let d = matrix[l1][l2];
            if d < min_dist {
                min_dist = d;
                res.clear();
                res.push(word2);
            } else if d == min_dist {
                res.push(word2);
            }
            last = word2;
            #[cfg(feature = "testing")]
            eprintln!();
        }

        results.push((word1, min_dist, res));
    }
    
    #[cfg(feature = "bench")]
    let algo_end = std::time::Instant::now();

    #[cfg(feature = "bench")]
    let out_start = std::time::Instant::now();
    for (a, b, c) in results {
        print!("{} ({})", a.chars().collect::<String>(), b);
        for word in c {
            print!(" {}", word.chars().collect::<String>());
        }
        println!();
    }
    #[cfg(feature = "bench")]
    let out_end = std::time::Instant::now();

    #[cfg(feature = "bench")]
    let total_end = std::time::Instant::now();

    #[cfg(feature = "bench")]
    #[cfg(feature = "sec")]
    {
        eprintln!("Alloc:   {:>10} ({:.9} s)", format!("{:?}", alloc_end - alloc_start), (alloc_end - alloc_start).as_secs_f32());
        eprintln!("In:      {:>10} ({:.9} s)", format!("{:?}", in_end - in_start), (in_end - in_start).as_secs_f32());
        eprintln!("Convert: {:>10} ({:.9} s)", format!("{:?}", convert_end - convert_start), (convert_end - convert_start).as_secs_f32());
        eprintln!("Algo:    {:>10} ({:.9} s)", format!("{:?}", algo_end - algo_start), (algo_end - algo_start).as_secs_f32());
        eprintln!("Out:     {:>10} ({:.9} s)", format!("{:?}", out_end - out_start), (out_end - out_start).as_secs_f32());
        eprintln!("Total:   {:>10} ({:.9} s)", format!("{:?}", total_end - tot_start), (total_end - tot_start).as_secs_f32());
    }
    #[cfg(feature = "bench")]
    #[cfg(not(feature = "sec"))]
    {
        eprintln!("Alloc:   {:>10}", format!("{:?}", alloc_end - alloc_start));
        eprintln!("In:      {:>10}", format!("{:?}", in_end - in_start));
        eprintln!("Convert: {:>10}", format!("{:?}", convert_end - convert_start));
        eprintln!("Algo:    {:>10}", format!("{:?}", algo_end - algo_start));
        eprintln!("Out:     {:>10}", format!("{:?}", out_end - out_start));
        eprintln!("Total:   {:>10}", format!("{:?}", total_end - tot_start));
    }
}
