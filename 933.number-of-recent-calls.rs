use std::collections::VecDeque;


struct RecentCounter {
    q: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            q: VecDeque::new()
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.q.push_back(t);
        while t - 3000 > self.q[0] {
            self.q.pop_front();
        }

        self.q.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
