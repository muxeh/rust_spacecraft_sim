# Rust Spacecraft Simulation

A Rust-based spacecraft simulation for modeling dynamics and control systems.

## Setup

### Prerequisites
- Rust (via [rustup](https://rustup.rs/))

### Installation
1. Clone the repo:
```
git clone https://github.com/muxeh/rust_spacecraft_sim.git
```

2. (For plotting) Set up Python environment and install dependencies:
```
cd rust_spacecraft_sim
python3 -m venv venv
source venv/bin/activate  # macOS/Linux
venv\Scripts\activate     # Windows
pip install -r requirements.txt
```

## Use
1. Build the project:
```
cd rust_spacecraft_sim
cargo build
```
2. Configure simulation case as desired in `configs/input.toml`
3. Run the simulation
```
cd rust_spacecraft_sim
cargo run
```
4. Plot simulation results
```
cd rust_spacecraft_sim/scripts
python3 plotter.py
```

## Contributing
Feel free to open issues or submit pull requests!
