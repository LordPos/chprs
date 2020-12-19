use super::Op;
use rand::random;

#[derive(Debug)]
struct Tableau {
    n : u32,
    tableau : Vec<bool>
    // a contiguous vector is more efficient than a vec of vecs because the latter includes dereferencing two pointers
}

macro_rules! at{
    ($self:ident, $i:expr, $j:expr) => {
        $self.tableau[($i * (2*$self.n + 1) + $j) as usize]
    };
}

// look! I wrote some code that writes code that writes code!
// repetitively accessing arbitrary indices isn't performant in rust because of bound-checks
// possible todo: replace with iterators
macro_rules! get_macros{
    ($self:ident) => {
        macro_rules! z{
            ($i:expr, $a:expr) => {
                at!($self, $i, $a + $self.n)
            }
        }
        macro_rules! x{
            ($i:expr, $a:expr) => {
                at!($self, $i, $a)
            }
        }
        macro_rules! r{
            ($i:expr) => {
                at!($self, $i, 2*$self.n)
            }
        }
    }
}

impl Tableau {
    fn new(n : u32) -> Tableau {
        Tableau {
            n : n,
            tableau : (0..4*n.pow(2) + 4*n + 1).map(|x|
                if x % (2*n + 1) == x / (2*n + 1) { true } else { false }
            ).collect() // sets diagonal elements to one
            // this forms the basis state. That is, |0〉^ ⊕n
        }
    }

    fn hadamard(&mut self, a : u32) {
        get_macros!(self);
        for i in 0..2*self.n {
            self.tableau.swap(
                (i * (2 * self.n + 1) + a) as usize,
                (i * (2 * self.n + 1) + a + self.n) as usize
            );
            r!(i) ^= x!(i, a) & z!(i, a);
        }
        // debugging println!("after hadamard {} : {:#?}", a, self.tableau)
    }

    fn phase(&mut self, a : u32) {
        get_macros!(self);
        for i in 0..2*self.n {
            r!(i) ^= x!(i, a) & z!(i, a);
            z!(i, a) ^= x!(i, a);
        }
    }

    fn cnot(&mut self, a : u32, b : u32) {
        get_macros!(self);
        for i in 0..2*self.n {
            r!(i) ^= x!(i, a) & z!(i, b) & (x!(i, b) ^ z!(i, b) ^ true);
            x!(i, b) ^= x!(i, a);
            z!(i, a) ^= z!(i, a);
        }
    }

    fn rowsum(&mut self, h : u32, i : u32) {
        get_macros!(self);
        let g = |x1, z1, x2 : bool, z2 : bool| match (x1, z1) {
            (false, false) => 0,
            (true, true) => z2 as i32 - x2 as i32,
            (true, false) => z2 as i32 * (2 * x2 as i32 - 1),
            (false, true) => x2 as i32 * (2 * z2 as i32 - 1)
        };

        let s= 2 * r!(h) as i32 +
            2 * r!(i) as i32 +
            (0..self.n).map(|j| 
                g(x!(i, j), z!(i, j), x!(h, j), z!(h, j))
            ).sum::<i32>();
        
        if s % 4 == 0 { r!(h) = false; } else { r!(h) = true; }

        for j in 0..self.n {
            x!(h, j) ^= x!(i, j);
            z!(h, j) ^= z!(i, j);
        }
    }

    fn measure(&mut self, a : u32) -> bool {
        get_macros!(self);
        let n = self.n;
        let mut p : Option<u32> = None;
        for i in n..2*n {
            if x!(i, a) { p = Some(i); break; }
        }

        if let Some(p) = p {
            for i in 0..2*n {
                if i != p && x!(i, a) {
                    self.rowsum(i, p);
                }
            }

            self.tableau.copy_within(
                (2 * p * n) as usize .. (p * (2*n + 1)) as usize,
                ((p - n)*(2*n + 1)) as usize
            );
            
            self.tableau.splice(
                (2 * p * n) as usize .. (p * (2*n + 1)) as usize,
                std::iter::repeat(false).take((2*n + 1) as usize)
            );

            r!(p) = random();
            z!(p, a) = true;
            // debugging println!("after measure {}: {:#?}", a, self.tableau);
            r!(p)

        } else {
            self.tableau.splice(
                (4*n.pow(2) + 2*n) as usize ..= (4*n.pow(2)+ 4*n) as usize,
                std::iter::repeat(false).take((2 * self.n + 1) as usize)
            );
            // debugging println!("after measure {}: {:#?}", a, self.tableau.len());
            for i in 0..n{
                if x!(i, a) {
                    self.rowsum(2*n, i + n)
                }
            }
            // debugging println!("after measure {}: {:#?}", a, self.tableau.len());
            r!(2*n)
        }
        
    }
}

pub fn chp(ops : Vec<Op>, qubits : u32) {
    let tableau = &mut Tableau::new(qubits);
    let l = tableau.tableau.len();
    tableau.tableau[l - 1] = false;
    for op in ops {
        match op {
            Op::Hadamard(a) => tableau.hadamard(a),
            Op::Phase(a) => tableau.phase(a),
            Op::Cnot(a, b) => tableau.cnot(a, b),
            Op::Measure(a) => println!("{}", tableau.measure(a))
        }
    }
}

