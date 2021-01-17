use std::collections::HashMap;

struct Chain {
    set: HashMap<u64, usize>
}

impl Chain {
    pub fn new(capacity: usize) -> Chain {
        let mut set = HashMap::with_capacity(capacity);
        set.insert(1, 1);
        Chain { set }
    }

    pub fn get_chain_len(&mut self, val: u64) -> usize {
        // Base case
        if let Some(res) = self.set.get(&val) {
            return *res;
        }
        // Otherwise recurse
        if val % 2 == 0 {
            // Getting result, then unwrapping to retrieve set, and get the len for this val
            let result = 1 + self.get_chain_len(val / 2);
            self.set.insert(val, result);
            return result;
        }
        else {
            // Odd, similar process
            // Can optimize by dividing 3 * val + 1 by 2, as it will always be even so can then divide by 2 at same time to remove step
            let result = 2 + self.get_chain_len((val * 3 + 1) / 2);
            self.set.insert(val, result);
            return result;
        }
    }
}

fn main() {
    let min: u64 = 99_999;
    let max: u64 = 999_999;

    let mut chain = Chain::new(max as usize);
    
    let result = (min..=max).map(|x| (x, chain.get_chain_len(x))).max_by_key(|x| x.1).unwrap();
    
    println!("{} ({})", result.0, result.1);
    
}

