use std::collections::VecDeque;

enum Funcs {
    F1(usize, usize, usize),
    
    F2(usize, usize, usize),
    F3(usize, usize, usize),
    B1(usize, usize, usize),
    B11(usize, usize, usize),
    B2(usize, usize, usize)
}

pub struct IncSeqIterator {
    a:Vec<u32>,
    stack:Vec<Funcs>,
    buffer:VecDeque<Vec<u32>>
}

impl IncSeqIterator {
    pub fn new(n:u32, k:u32) -> Self {
        let mut a = vec![0; (n+1) as usize];
        for j in 1..=k {
            a[(n-k+j) as usize] = j-1;
        }
        
        let mut buffer = VecDeque::new();
        if k == 0 {
            if n == 0 {
                // For n = k = 0 there exists the empty partition, so
                // we need to return a single empty vector.
                buffer.push_back(Vec::new()); 
            }
            return IncSeqIterator{a, stack: Vec::new(), buffer};
        } 

        if k == 1 {
            let mut res = IncSeqIterator{a, stack: Vec::new(), buffer};
            res.visit(); // Put single sequence into buffer
            return res;
        }

        let stack = vec![Funcs::F1(k as usize, n as usize, 0)];
        IncSeqIterator{a, stack, buffer}
    }

    fn run(&mut self) -> bool {
        match self.stack.pop() {
            Some(Funcs::F1(mu, nu, sigma)) => {
                self.f1(mu, nu, sigma);
                true
            }
            Some(Funcs::F2(mu, nu, sigma)) => {
                self.f2(mu, nu, sigma);
                true
            }
            Some(Funcs::F3(mu, nu, sigma)) => {
                self.f3(mu, nu, sigma);
                true
            }
            Some(Funcs::B1(mu, nu, sigma)) => {
                self.b1(mu, nu, sigma);
                true
            }
            Some(Funcs::B11(mu, nu, sigma)) => {
                self.b11(mu, nu, sigma);
                true
            }
            Some(Funcs::B2(mu, nu, sigma)) => {
                self.b2(mu, nu, sigma);
                true
            }
            None => false,
        }
    }    
    
    fn visit(&mut self) {
        let res: Vec<u32> = self.a.iter().skip(1).cloned().collect();
        self.buffer.push_back(res);
    }
    
    fn f1(&mut self, mu:usize, nu:usize, sigma:usize) {
        self.stack.push(Funcs::F2(mu, nu, sigma)); // Continue at F2 afterwards
        if mu == 2 {
            self.visit()
        } else {
            // Recursive call to f
            self.stack.push(Funcs::F1(mu-1, nu-1, (mu+sigma)%2));
        }
    }

    fn f2(&mut self, mu:usize, nu:usize, sigma:usize) {
        if nu == mu + 1 {
            // We are done with this call to f so 
            // nothing gets added to the stack
            self.a[mu] = (mu - 1) as u32;
            self.visit();
            while self.a[nu] > 0 {
                self.a[nu] = self.a[nu] - 1;
                self.visit();
            }
        } else if nu > mu + 1 {
            if (mu + sigma) % 2 == 1 {
                self.a[nu - 1] = (mu - 1) as u32;
            } else {
                self.a[mu] = (mu - 1) as u32;
            }

            // Continue at f3 afterwards
            self.stack.push(Funcs::F3(mu, nu, sigma));
            if (self.a[nu] + sigma as u32) % 2 == 1 {
                self.stack.push(Funcs::B1(mu, nu-1, 0));
            } else {
                self.stack.push(Funcs::F1(mu, nu-1, 0));
            }
        } 
        // Otherwise we are done with f
    }    

    fn f3(&mut self, mu:usize, nu:usize, sigma:usize) {
        if self.a[nu] > 0 {
            // This simulates a loop until self.a[nu] is zero
            self.stack.push(Funcs::F3(mu, nu, sigma));

            // We make some recursive calls while modifying self.a
            self.a[nu] = self.a[nu] - 1;
            if (self.a[nu] + sigma as u32) % 2 == 1 {
                self.stack.push(Funcs::B1(mu, nu - 1, 0));
            } else {
                self.stack.push(Funcs::F1(mu, nu - 1, 0));
            }
        }
    }    
    
    fn b1(&mut self, mu:usize, nu:usize, sigma:usize) {
        // Continue at b2 afterwards
        self.stack.push(Funcs::B2(mu, nu, sigma)); 
        if nu == mu + 1 {
            while self.a[nu] < (mu - 1) as u32 {
                self.visit();
                self.a[nu] += 1;
            }
            self.visit();
            self.a[mu] = 0;
        } else if nu > mu + 1 {
            // Continue at b11 afterwards
            self.stack.push(Funcs::B11(mu, nu, sigma));
            if (self.a[nu] + sigma as u32) % 2 == 1 {
                self.stack.push(Funcs::F1(mu, nu-1, 0));
            } else {
                self.stack.push(Funcs::B1(mu, nu-1, 0));
            }
        }
    }
    
    fn b11(&mut self, mu:usize, nu:usize, sigma:usize) {
        if self.a[nu] < (mu - 1) as u32 {
            // This simulates a loop until a[nu] = mu - 1
            self.stack.push(Funcs::B11(mu, nu, sigma));
            self.a[nu] +=  1;
            if (self.a[nu] + sigma as u32) % 2 == 1 {
                self.stack.push(Funcs::F1(mu, nu-1, 0));
            } else {
                self.stack.push(Funcs::B1(mu, nu-1, 0));
            }
        } else {
            if (mu + sigma) % 2 == 1 {
                self.a[nu - 1] = 0;
            } else {
                self.a[mu] = 0;
            }
        }
    }
    
    fn b2(&mut self, mu:usize, nu:usize, sigma:usize) {
        if mu == 2 {
            self.visit();
        } else {
            self.stack.push(Funcs::B1(mu-1, nu-1, (mu+sigma)%2 ));
        }
    }        
}

impl Iterator for IncSeqIterator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        // While the buffer is empty we call run() to fill
        // it up. run() will return 'false' once the stack is 
        // exhausted.
        while self.buffer.len() == 0 && self.run() {
            ()
        }

        // If we are here and the buffer is empty it is because
        // we are done iterating.
        return self.buffer.pop_front();
    }
}
