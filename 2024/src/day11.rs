use crate::common;
use std::io;
use std::process::exit;

//#[cfg(debug_assertions)]
//use std::{thread, time};

use std::collections::HashMap;

/*
pub struct Cache {
    func: Box<dyn Fn(u64) -> [u64; 2]>,
    store: RefCell<HashMap<u64, [u64; 2]>>,
}

impl Cache {
    pub fn new(func: Box<dyn Fn(u64) -> [u64; 2]>) -> Cache {
        Self {
            func,
            store: RefCell::new(HashMap::new()),
        }
    }

    pub fn value(&mut self, arg: u64) -> [u64; 2] {
        if let Some(value) = self.store.borrow().get(&arg) {
            return *value;
        };
        let result = (self.func)(arg);
        self.store.borrow_mut().insert(arg, result);
        return result;
    }
}

struct Rules {
    cache_2: Rc<RefCell<Cache>>,
    cache_3: Rc<RefCell<Cache>>,
}

impl Rules {
    fn rule_2_calc(stone: u64) -> [u64; 2] {
        let exp = stone.ilog10() + 1;
        if exp % 2 == 0 {
            let pow = (10 as u64).pow(exp / 2);
            let first_stone: u64 = stone / pow;
            let second_stone = stone - first_stone * pow;
            return [first_stone, second_stone];
        }
        return [0, 0];
    }

    fn rule_3_calc(stone: u64) -> [u64; 2] {
        return [stone * 2024, 0];
    }

    fn new() -> Rules {
        return Rules {
            cache_2: Rc::new(RefCell::new(Cache::new(Box::new(Rules::rule_2_calc)))),
            cache_3: Rc::new(RefCell::new(Cache::new(Box::new(Rules::rule_3_calc)))),
        };
    }

    fn eval(&mut self, stone: u64, n: u8) -> Box<dyn Iterator<Item = u64>> {
        return Rules::process(stone, n, self.cache_2.clone(), self.cache_3.clone());
    }

    fn process<'a>(
        stone: u64,
        n: u8,
        cache_2: Rc<RefCell<Cache>>,
        cache_3: Rc<RefCell<Cache>>,
    ) -> Box<dyn Iterator<Item = u64> + 'a> {
        if n == 0 {
            return Box::new(std::iter::once(stone));
        }
        if stone == 0 {
            return Box::new((0..1).flat_map(move |s| {
                Rules::process(s as u64, n - 1, cache_2.clone(), cache_3.clone())
            }));
        }
        let new_stones = cache_2.borrow_mut().value(stone);
        if new_stones[0] != 0 {
            return Box::new(
                new_stones
                    .into_iter()
                    .flat_map(move |s| Rules::process(s, n - 1, cache_2.clone(), cache_3.clone())),
            );
        }
        let stone = cache_3.borrow_mut().value(stone)[0];
        return Box::new(
            [stone]
                .into_iter()
                .flat_map(move |s| Rules::process(s, n - 1, cache_2.clone(), cache_3.clone())),
        );
    }
}

fn blinks(stones: &[u64], n: u8) -> usize {
    // parallel execution
    //let num_threads = std::thread::available_parallelism().unwrap().get();
    //let mut handles = Vec::with_capacity(num_threads);
    //let chunk_size = (stones.len() + num_threads - 1) / num_threads;
    let mut rules = Rules::new();
    //for chunk in stones.chunks(chunk_size) {
    //    let chunk = chunk.to_vec();
    //    handles.push(thread::spawn(move || {
    //        let mut sum: usize = 0;
    //        for stone in chunk {
    //            sum += rules.rule_1(stone, n).count();
    //        }
    //        sum
    //    }))
    //}
    let mut sum = 0;
    //for handler in handles {
    //    sum += handler.join().unwrap();
    //}
    for stone in stones {
        sum += rules.eval(*stone, n).count();
        dbg!("Stone {} finished, sum {}", stone, sum);
    }
    return sum;
}
 */

#[derive(Debug, Clone)]
struct Rules;

impl Rules {
    // Apply rule for a single stone
    fn apply(stone: u64) -> Vec<u64> {
        if stone == 0 {
            return vec![1]; // Rule 1: If stone is 0, it becomes 1
        }

        let digits = stone.to_string();
        let len = digits.len();

        // Rule 2: If the stone has even digits, split it
        if len % 2 == 0 {
            let mid = len / 2;
            let first_half: u64 = digits[..mid].parse().unwrap();
            let second_half: u64 = digits[mid..].parse().unwrap();
            return vec![first_half, second_half]; // Split into two stones
        }

        // Rule 3: Else, multiply the stone by 2024
        vec![stone * 2024] // Apply rule 3
    }

    fn heat_cache(stone: u64, cache: &mut HashMap<u64, Vec<u64>>) {
        if !cache.get(&stone).is_none() {
            return;
        }
        let next_stones = Self::apply(stone);
        cache.insert(stone, next_stones.clone());

        // Apply recursively for each next stone after the current step
        for next_stone in next_stones.iter() {
            Self::heat_cache(*next_stone, cache);
        }
    }

    fn process(
        stone: u64,
        n: u8,
        cache: &std::sync::Arc<HashMap<u64, Vec<u64>>>,
    ) -> Box<dyn Iterator<Item = u64> + '_> {
        if n == 0 {
            return Box::new(std::iter::once(stone));
        }
        return Box::new(
            cache
                .get(&stone)
                .unwrap()
                .iter()
                .flat_map(move |s| Rules::process(*s, n - 1, cache)),
        );
    }
}

// Function to calculate stones after n steps
fn calculate_stones(
    initial_stones: &Vec<u64>, // The initial array of stones
    n: u8,                     // The number of steps left
) -> usize {
    let mut cache: HashMap<u64, Vec<u64>> = HashMap::new(); // Cache for storing evolutions
    for stone in initial_stones {
        Rules::heat_cache(*stone, &mut cache);
    }
    let mut stone_count: usize = 0;

    // parallel execution
    let num_threads = std::thread::available_parallelism().unwrap().get();
    let mut handles = Vec::with_capacity(num_threads);
    let chunk_size = (initial_stones.len() + num_threads - 1) / num_threads;
    let sync_cache = std::sync::Arc::new(cache);
    if initial_stones.chunks(chunk_size).len() < num_threads - 3 || n == 0 {
        let stones: Vec<u64> = initial_stones
            .iter()
            .flat_map(|s| Rules::process(*s, 1, &sync_cache))
            .collect::<Vec<_>>();
        return calculate_stones(&stones, n - 1);
    }
    dbg!(
        "Precalculated n, chunks",
        n,
        initial_stones.chunks(chunk_size).len()
    );
    for chunk in initial_stones.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let cache = std::sync::Arc::clone(&sync_cache);
        handles.push(std::thread::spawn(move || {
            let mut sum: usize = 0;
            for stone in chunk {
                sum += Rules::process(stone, n, &cache).count();
            }
            sum
        }))
    }
    for handler in handles {
        stone_count += handler.join().unwrap();
    }
    // For each initial stone, calculate its evolution and update the stone count
    //for stone in initial_stones {
    //    stone_count += Rules::process(*stone, n, &cache).count();
    //}

    stone_count
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        eprintln!("Usage day2 <input-file> <number-of-blinks>");
        exit(1);
    }

    let file_name = args[0].as_str();
    let n = args[1].parse::<u8>().expect("Second argument must be int");

    let lines: Vec<Vec<u64>> = common::parse_file(file_name)?;
    println!(
        "Here are stones after {} blinks: {}",
        calculate_stones(&lines[0], n),
        n
    );

    Ok(())
}
