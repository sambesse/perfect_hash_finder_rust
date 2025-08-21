use rand::Rng;
use std::time::Instant;
use std::vec;

const P: u32 = 66587;
const K_MAX: u32 = 0xfffff;
const N_PERF_TEST: u32 = 10000;

struct MessageDesc {
    id: u16,
    tag: String,
}

impl MessageDesc {
    fn new(id: u16) -> Self {
        MessageDesc {
            id: id,
            tag: id.to_string(),
        }
    }
}

struct PerfectHash {
    p: u32,
    k: u32,
    m: u32,
}

fn main() -> () {
    let mut can_ids = vec![];
    can_ids.push(MessageDesc::new(0x16));
    can_ids.push(MessageDesc::new(0x23));
    can_ids.push(MessageDesc::new(0x203));
    can_ids.push(MessageDesc::new(0x301));
    can_ids.push(MessageDesc::new(0x600));
    can_ids.push(MessageDesc::new(0x601));
    can_ids.push(MessageDesc::new(0x602));

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
                perf_test(perf_hash, hash_table, can_ids);
                return ();
            }
            k += 1;
            for item in &mut hash_table {
                *item = None;
            }
        }
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
    msg_list: Vec<MessageDesc>,
) {
    let mut rng = rand::thread_rng();
    println!("perf testing using perfect hash");
    let mut total_duration = 0u128;
    for _n in 1..N_PERF_TEST {
        let rand_id = rng.gen_range(0..0xfff) as u16;
        let begin = Instant::now();
        let index = hash(hash_desc.m, hash_desc.k, &rand_id);
        let _res = hash_table.get(index as usize);
        let duration = begin.elapsed().as_nanos();
        total_duration += duration;
    }
    let avg_duration = total_duration / N_PERF_TEST as u128;
    println!("custom hashing algo average duration: {avg_duration}");
}

fn naive_search(id: u16, msg_list: Vec<MessageDesc>) -> Option<MessageDesc> {
    for _n in 1..msg_list.len() {
        if msg_list[_n].id == id {
            return Some(MessageDesc::new(id));
        }
    }
    return None;
}
