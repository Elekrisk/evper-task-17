use std::{hint::unreachable_unchecked, io::{BufRead, Read, stdin}, ops::{Index, IndexMut}};

#[derive(Copy, Clone)]
struct SVecC {
    inner: [u8; 80],
    len: usize
}

impl SVecC {
    pub fn new() -> Self {
        Self {
            inner: [0; 80],
            len: 0
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&u8> {
        self.inner[0..self.len].iter()
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
    let start = std::time::Instant::now();


    let mut word_list: Vec<SVecC> = Vec::with_capacity(500_000);
    let mut misspelled: Vec<SVecC> = Vec::with_capacity(100);

    let mut stdin = std::io::stdin();
    let mut input = String::with_capacity(201_000_000);
    stdin.read_to_string(&mut input);
    //let mut lines = input.lines();
    let mut bytes = input.bytes();

    // FOR DEBUGGING PURPOSES
    #[cfg(feature = "testing")]
    let input_file = std::env::args().skip(1).next().expect("path to test file expected");
    #[cfg(feature = "testing")]
    let input = std::fs::read_to_string(input_file).unwrap();
    #[cfg(feature = "testing")]
    let mut lines = input.lines().map(|l| Result::<_, ()>::Ok(l.to_string()));

    let mut buffer = SVecC::new();
    loop {
        if let Some(c) = bytes.next() {
            match c {
                b'#' => break,
                b'\n' => {
                    word_list.push(buffer);
                    buffer.clear();
                },
                #[cfg(windows)]
                b'\r' => {},
                0xc3 => buffer.push(match bytes.next() { Some(v) => v, None => unsafe { unreachable_unchecked() } }),
                o => buffer.push(o)
            }
        } else {
            break
        }
    }
    #[cfg(windows)]
    bytes.next();
    bytes.next();

    loop {
        if let Some(c) = bytes.next() {
            match c {
                b'\n' => {
                    misspelled.push(buffer);
                    buffer.clear();
                },
                #[cfg(windows)]
                b'\r' => {},
                0xc3 => buffer.push(match bytes.next() { Some(v) => v, None => unsafe { unreachable_unchecked() } }),
                o => buffer.push(o)
            }
        } else {
            break;
        }
    }


    let mut matrix = [[0; 41]; 41];
    for i in 1..41 {
        matrix[i][0] = i as u8;
        matrix[0][i] = i as u8;
    }

    
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

            #[cfg(feature = "testing")]
            eprintln!("   {}", word2.iter().map(|c| vec![' ', ' ', *c]).flatten().collect::<String>());

            for p1 in 1..=word1.len() {
                #[cfg(feature = "testing")]
                eprint!("  {}", word1[p1 - 1]);
                #[cfg(feature = "testing")]
                for i in 1..start {
                    eprint!("{:3}", matrix[p1][i]);
                }

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
        print!("{} ({})", word1.chars().collect::<String>(), min_dist);
        for word in res {
            print!(" {}", word.chars().collect::<String>());
        }
        println!();
    }

    #[cfg(feature = "bench")]
    {
        let end = std::time::Instant::now();
        eprintln!("Time: {:?}", end - start);
    }
}
