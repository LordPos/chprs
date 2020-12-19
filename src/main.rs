mod chp;
use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

#[derive(Debug)]
// enumeration for possible operations
// u32 is the fastest, but it also sets the qubit limit at 2^32
// not worrying too much about it though, it's unlikely that we can simulate over a few thousands anyway
pub enum Op {
    Hadamard(u32),
    Phase(u32),
    Measure(u32),
    Cnot(u32,u32)
}


fn main() {
    let name : String = env::args()
        .nth(1)
        .expect("file name not provided");
    
    let lines = BufReader::new(
        File::open(name)
            .unwrap_or_else(|err| panic!("error opening file: {}", err)))
        .lines();        
    
    let mut ops : Vec<Op> = vec![];
    let mut num_qubits = 0u32;
    for (line_no, line) in lines.enumerate() {
        match line
            .unwrap_or("".to_string())
            .split(' ')
            .collect::<Vec<&str>>()
            .as_slice() {
            ["h", q] | ["H", q] => { // repetitive code. replace with a macro?
                let q = q.parse::<u32>().unwrap_or_else(|err|
                    panic!("error parsing line {} : {}", line_no, err)
                );
                if q > num_qubits { num_qubits = q;}
                ops.push(Op::Hadamard(q));
            },
            ["p", q] | ["P", q] => {
                let q = q.parse::<u32>().unwrap_or_else(|err|
                    panic!("error parsing line {} : {}", line_no, err)
                );
                if q > num_qubits { num_qubits = q;}
                ops.push(Op::Phase(q));
            },
            ["m", q] | ["M", q] => {
                let q = q.parse::<u32>().unwrap_or_else(|err|
                    panic!("error parsing line {} : {}", line_no, err)
                );
                if q > num_qubits { num_qubits = q;}
                ops.push(Op::Measure(q));
            }
            ["c", a, b] | ["C", a, b] => {
                let a = a.parse::<u32>().unwrap_or_else(|err|
                    panic!("error parsing line {} : {}", line_no, err)
                );
                let b = b.parse::<u32>().unwrap_or_else(|err|
                    panic!("error parsing line {} : {}", line_no, err)
                );
                if a > num_qubits { num_qubits = a;}
                if b > num_qubits { num_qubits = b;}
                ops.push(Op::Cnot(a,b));
            },
            _ => continue
        }
    }

    chp::chp(ops, num_qubits + 1);
    // num_qubits stores the highest *index*, which means the actual number is num_qubits + 1
}

