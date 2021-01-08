# CHP.rs
### An implementation of [Gottesman-Knill](https://en.wikipedia.org/wiki/Gottesman%E2%80%93Knill_theorem) quantum simulation in rust
* * *


### What?
The Gottesman-Knill theorem (or, more precisely, the algorithm implied by it) makes the simulation of qubits possible in _polynomial_ time and space complexity as opposed to the exponential complexity of full-fledged quantum simulators on classical machines, but there's a caveat — Only the [Clifford set](https://en.wikipedia.org/wiki/Clifford_gates) of gates can be simulated. Quantum circuits built with only the Clifford set are called stabilizer circuits.	

I initially set out to make a full quantum simulator in nim before I stumbled upon this, which seemed less trivial. Note that this implementation uses [Scott Aaronson's Improved simulation of stabilizer circuits](https://arxiv.org/abs/quant-ph/0406196) instead of the original algorithm. A C implementation (that's literally older than me) called CHP (for CNOT-Hadamard-Phase, the most used Clifford gates) is also provided with the paper

### Motivation behind the project

As I mentioned earlier, the C version is not exactly new. The author only had access to 256MB of RAM when it was written [or so the paper said]. Thousands of qubits could be simulated with that. Fast-forward to today where I — a nobody — have 16GB of RAM and 16 threads. Modern multiprocessing capabilities make the entire process so much faster, while the increased memory allows for more qubits. Perhaps we can simulate and measure hundreds of thousands of qubits in lesser time now. If that doesn't seem impressive compare that with the ~20 qubits that can be fully simulated on a typical classical machine.

That's the main motivation behind this project: add concurrency support. Rust, as a low-level language that emphasizes safety and has amazing multithreading support was an obvious choice. (also I've been itching to try it out)

### Usage

Very, very beta at the moment. After `git clone`ing `cd`ing into the directory, run
```
cargo run filename.chp
```
replacing filename with the disired file's name. Chprs uses the same .chp file format as [the original](https://www.scottaaronson.com/chp/).

### Roadmap 

- Track down the last few bugs
- Make an actual UI. The current one is extremely minimal
- Add concurrency support (in my defense, _time_)
- documentation and perhaps a few more features
- Benchmark!
