use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;
use std::{env, vec};

const P: u32 = 66587;
const K_MAX: u32 = 0xfffff;
const N_PERF_TEST: u32 = 10000;
const MAX_CAN_ID: u16 = 0x610;

struct MessageDesc {
    id: u16,
    _tag: String,
}

impl MessageDesc {
    fn new(id: u16) -> Self {
        MessageDesc {
            id: id,
            _tag: id.to_string(),
        }
    }
}

struct PerfectHash {
    p: u32,
    k: u32,
    m: u32,
}

fn print_usage(prog_name: &String) {
    println!("{prog_name} <number_can_ids>")
}

fn main() -> () {
    let mut can_ids = vec![];
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("too few arguments provided");
        print_usage(&args[0]);
        return ();
    }

    let num_args: u32 = args[1].parse().unwrap();
    let mut hash_map = HashMap::new();
    let mut rng = rand::thread_rng();
    for _i in 0..num_args {
        let mut rand_id = rng.gen_range(0..MAX_CAN_ID) as u16;
        while hash_map.contains_key(&rand_id) {
            println!("found duplicate trying again");
            rand_id = rng.gen_range(0..MAX_CAN_ID) as u16;
        }
        let new_id = rand_id;
        let new_msg = MessageDesc::new(rand_id);
        hash_map.insert(new_id, new_msg);
        can_ids.push(MessageDesc::new(new_id));
    }

    let mut m = can_ids.len() as u32;
    let mut k = 1u32;
    let mut hash_table: Vec<Option<MessageDesc>> = Vec::new();
    let mut n = 0;
    while n < m {
        hash_table.push(None);
        n += 1;
    }

    loop {
        while k < K_MAX {
            let mut i = 0;
            while i < can_ids.len() {
                let index = hash(m, k, &can_ids[i].id);
                match hash_table.get(index as usize) {
                    None => {
                        break;
                        // hash_table.replace(index as usize, Some(MessageDesc::new(can_ids[i].id)))
                    }
                    Some(_i) => match _i {
                        None => hash_table[index as usize] = Some(MessageDesc::new(can_ids[i].id)),
                        Some(_j) => break,
                    },
                };
                i += 1;
            }
            if i == can_ids.len() {
                println!("no collisions");
                let perf_hash = PerfectHash { p: P, k: k, m: m };
                println!(
                    "parameters k: {}, m: {} p: {}",
                    perf_hash.k, perf_hash.m, perf_hash.p
                );
                /* TODO: how to print vec cleanly without borrowing nightmare */
                println!("hash table:");
                for item in &hash_table {
                    match item {
                        None => println!("0"),
                        Some(message_desc) => println!("{:x}", message_desc.id),
                    };
                }
                perf_test(perf_hash, hash_table, &can_ids, hash_map);
                return ();
            }
            k += 1;
            for item in &mut hash_table {
                *item = None;
            }
        }
        println!("no suitable k found for m = {m}");
        k = 0;
        m += 1;
        hash_table.push(None);
    }
}

fn hash(m: u32, k: u32, x: &u16) -> u32 {
    ((k * (*x as u32)) % P) % m
}

fn perf_test(
    hash_desc: PerfectHash,
    hash_table: Vec<Option<MessageDesc>>,
    msg_list: &Vec<MessageDesc>,
    hash_map: HashMap<u16, MessageDesc>,
) {
    let mut rng = rand::thread_rng();
    println!("perf testing using perfect hash");
    let mut custom_hash_duration = 0u128;
    let mut naive_duration = 0u128;
    let mut builtin_hash_duration = 0u128;
    for _n in 1..N_PERF_TEST {
        let rand_id = rng.gen_range(0..0xfff) as u16;
        let begin = Instant::now();
        let index = hash(hash_desc.m, hash_desc.k, &rand_id);
        let _res = hash_table.get(index as usize);
        let duration = begin.elapsed().as_nanos();
        custom_hash_duration += duration;
        let begin = Instant::now();
        let _res = naive_search(rand_id, &msg_list);
        let duration = begin.elapsed().as_nanos();
        naive_duration += duration;
        let begin = Instant::now();
        let _res = native_search(rand_id, &hash_map);
        let duration = begin.elapsed().as_nanos();
        builtin_hash_duration += duration;
    }
    let avg_duration = custom_hash_duration / N_PERF_TEST as u128;
    println!("custom hashing algo average duration: {avg_duration}");
    let avg_duration = naive_duration / N_PERF_TEST as u128;
    println!("naive algo average duration: {avg_duration}");
    let avg_duration = builtin_hash_duration / N_PERF_TEST as u128;
    println!("builtin hashing algo average duration: {avg_duration}");
}

fn naive_search(id: u16, msg_list: &Vec<MessageDesc>) -> Option<MessageDesc> {
    for _n in 1..msg_list.len() {
        if msg_list[_n].id == id {
            return Some(MessageDesc::new(id));
        }
    }
    return None;
}

fn native_search(id: u16, msg_list: &HashMap<u16, MessageDesc>) -> Option<MessageDesc> {
    let ret = msg_list.get(&id);
    match ret {
        None => return None,
        Some(op) => return Some(MessageDesc::new(op.id)),
    }
}
