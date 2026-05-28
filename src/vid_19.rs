#![allow(dead_code)]
use std::sync::{Arc, Mutex};

// A RateLimiter should control how many requests can be made.
// It uses a Mutex to ensure that the count is updated safely across threads,
// and an Arc to allow the counter to be shared.
#[derive(Clone)]
struct RateLimiter {
    max_req: u16,
    curr_req: Arc<Mutex<u16>>,
}

// ** START EDITS HERE **

impl RateLimiter {
    fn new(max_req: u16) -> Self {
        // Initialize RateLimiter. curr_req should start at 0.
        // Remember to wrap the Mutex in an Arc!
        RateLimiter {
            max_req,
            curr_req: Arc::new(Mutex::new(0)),
        }
    }

    fn allow_request(&self) -> bool {
        // 1. Lock the mutex to get access to the current request count.
        // 2. Check if the count is less than max_req_per_sec.
        // 3. If it is, increment the count and return true.
        // 4. Otherwise, return false.
        let mut count = self.curr_req.lock().unwrap();
        if *count < self.max_req {
            *count += 1;
            true
        } else {
            false
        }
    }
}

// ** END EDITS HERE **

#[cfg(test)]
mod tests {
    use super::RateLimiter;
    use std::thread;

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(5);
        let mut handles = vec![];

        for _ in 0..10 {
            let limiter_clone = limiter.clone();
            let handle = thread::spawn(move || limiter_clone.allow_request());
            handles.push(handle);
        }

        let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        let success_count = results.iter().filter(|&&res| res).count();

        assert_eq!(success_count, 5);
        assert_eq!(*limiter.curr_req.lock().unwrap(), 5);
    }
}
