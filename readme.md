# Line Coding Simulator

## Overview

The Line Coding Simulator is a Rust-based command-line application that allows users to encode binary strings using various line coding schemes, analyze the resulting signal, and visualize it. It is designed to help students and researchers understand the properties and performance of different line coding techniques.

### Features

* Supported Line Coding Schemes:
* Unipolar NRZ: Non-return-to-zero with positive voltage for 1 and zero voltage for 0.
* Polar NRZ: Non-return-to-zero with positive voltage for 1 and negative voltage for 0.
* Manchester: Self-synchronizing scheme with transitions in the middle of each bit period.
* AMI (Alternate Mark Inversion): Balanced encoding where 1 alternates between positive and negative voltages, and 0 is zero voltage.
* Metrics Analysis:
* Average Power: Mean square of signal amplitudes.
* DC Component: Average voltage to evaluate signal bias.
* Synchronization Capability: Number of transitions in the signal.
* Visualization:
* Graphical representation of the encoded signal saved as a PNG file.

#### Run

```bash
cargo run
```
