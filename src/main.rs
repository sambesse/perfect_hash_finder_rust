use std::vec;

const P: u32 = 66587;
const K_MAX: u32 = 0xfffff;

fn main() -> ! {
    let can_ids = vec![0x23u16, 0x306, 0x600, 0x601, 0x602, 0xa, 0x5a, 0x313];

    let mut m = can_ids.len() as u32;
    let mut k = 1u32;

    loop {
        let mut hash_table = vec![0u16; m as usize];
        while k < K_MAX {
            let i = 0;
            while i < can_ids.len() {
                let index = hash(m, k, &can_ids[i]);
                if hash_table[index as usize] == 0u16 {
                    hash_table[index as usize] = can_ids[i];
                } else {
                    break;
                }
            }
            println!("no collisions");
            println!("parameters k: {}, m: {}", k, m);
            /* TODO: how to print vec cleanly without borrowing nightmare */
        }
        k = 0;
        m += 1;
    }
}

fn hash(m: u32, k: u32, x: &u16) -> u32 {
    ((k * (*x as u32)) % P) % m
}
