# ternary-fuzzy

Fuzzy logic with ternary membership — fuzzy sets, membership functions, inference rules, and defuzzification using a three-valued membership scale.

## Why This Exists

Classical fuzzy logic uses continuous membership degrees in [0, 1]. This precision comes at a cost: complex defuzzification, floating-point sensitivity, and over-engineering for problems that really just need "low / medium / high" distinctions.

**ternary-fuzzy** replaces the continuous membership function with a three-valued scale: `Low` (-1), `Medium` (0), `High` (+1). This simplifies every part of the fuzzy pipeline — membership evaluation is a threshold, t-norms are min, t-conorms are max, complement is symmetric, and defuzzification is just averaging. The result is a fuzzy system that runs on fixed-point or integer-only hardware, produces interpretable outputs, and still captures the essential "degrees of truth" that binary logic misses.

## Core Concepts

| Type | Meaning |
|---|---|
| `TernaryMembership` | Membership degree: `Low` (-1), `Medium` (0), `High` (+1) |
| `TernaryFuzzySet` | A set mapping elements to ternary membership |
| `MembershipFunction` | Trait: maps a crisp value to ternary membership |
| `TriangularTernaryMF` | Triangular shape: Low at edges, High at peak, Medium in between |
| `StepTernaryMF` | Step thresholds: below → Low, between → Medium, above → High |
| `FuzzyRule` | IF antecedents THEN consequent, with AND/OR operators |
| `FuzzyControlSystem` | Complete fuzzy controller: fuzzify → rules → defuzzify |

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-fuzzy = "0.1"
```

```rust
use ternary_fuzzy::*;
use std::collections::HashMap;

fn main() {
    // Build a fuzzy controller
    let mut sys = FuzzyControlSystem::new();

    // Define input membership functions
    sys.add_input("temperature", Box::new(StepTernaryMF::new(30.0, 70.0)));
    sys.add_input("humidity", Box::new(StepTernaryMF::new(40.0, 80.0)));

    // Add rules
    sys.add_rule(FuzzyRule::new_and(
        vec![
            ("temperature".into(), TernaryMembership::High),
            ("humidity".into(), TernaryMembership::High),
        ],
        ("fan_speed".into(), TernaryMembership::High),
    ));
    sys.add_rule(FuzzyRule::new_and(
        vec![("temperature".into(), TernaryMembership::Low)],
        ("fan_speed".into(), TernaryMembership::Low),
    ));

    // Evaluate
    let mut inputs = HashMap::new();
    inputs.insert("temperature".into(), 80.0);
    inputs.insert("humidity".into(), 90.0);

    let output = sys.evaluate(&inputs);
    println!("Fan speed: {:?}", output.get("fan_speed"));
    // Fan speed: Some(High)
}
```

## API Overview

### TernaryMembership
- `Low` (-1), `Medium` (0), `High` (+1)
- `value() → i32`, `from_i32(v)` — numeric conversion

### TernaryFuzzySet
- `new(name)`, `insert(element, membership)`, `membership(element)` — basic operations
- `support()` — elements with non-Low membership
- `core()` — elements with High membership

### Membership Functions
- `TriangularTernaryMF::new(low, mid, high)` — triangular shape
- `StepTernaryMF::new(low_threshold, high_threshold)` — step thresholds
- Implement `MembershipFunction` trait for custom shapes

### Fuzzy Operators
- `ternary_tnorm(a, b)` — fuzzy AND (min)
- `ternary_tconorm(a, b)` — fuzzy OR (max)
- `ternary_complement(a)` — fuzzy NOT (swap Low ↔ High, Medium stays)

### FuzzyRule
- `new_and(antecedents, consequent)` — AND rule
- `new_or(antecedents, consequent)` — OR rule
- `firing_strength(memberships) → TernaryMembership` — evaluate rule

### FuzzyControlSystem
- `add_input(name, mf)`, `add_rule(rule)` — configure
- `evaluate(inputs) → HashMap<String, TernaryMembership>` — run full pipeline

### Defuzzification
- `defuzzify_to_ternary(values) → TernaryMembership` — average and threshold

## How It Works

**Fuzzification** maps each crisp input through its membership function to a `TernaryMembership`. `StepTernaryMF` uses two thresholds: values below the low threshold are `Low`, above the high threshold are `High`, and everything in between is `Medium`. `TriangularTernaryMF` maps the peak to `High`, the edges to `Low`, and the slopes to `Medium`.

**Rule evaluation** computes the firing strength of each rule by applying t-norms (AND → min) or t-conorms (OR → max) across antecedent memberships. The firing strength is combined with the consequent membership via t-norm, producing a qualified output.

**Defuzzification** collects all qualified outputs for each variable and averages their numeric values (-1, 0, +1). If the average exceeds ±0.33, it maps to High or Low; otherwise Medium. This is far simpler than centroid or mean-of-maxima defuzzification in classical fuzzy logic, and produces crisp ternary decisions.

## Use Cases

- **HVAC control** — map temperature and humidity to fan speed (Low/Medium/High) with interpretable rules
- **Industrial quality control** — classify sensor readings into Low/Medium/High quality, aggregate across multiple inspection stations
- **Embedded decision-making** — run fuzzy inference on integer-only microcontrollers for simple ternary actuation

## Ecosystem

Part of the **SuperInstance** ternary computing ecosystem:

- [`ternary`](https://crates.io/crates/ternary) — core trit types and balanced ternary arithmetic
- [`ternary-fuzzy`](https://crates.io/crates/ternary-fuzzy) — this crate
- [`ternary-control`](https://crates.io/crates/ternary-control) — ternary PID and bang-bang controllers
- [`ternary-sensor`](https://crates.io/crates/ternary-sensor) — sensor classification and fusion
- [`ternary-kalman`](https://crates.io/crates/ternary-kalman) — Kalman filtering for ternary states

## Known Limitations

- **Three-valued membership loses granularity**: Membership functions map continuous inputs to just three levels (Low/Medium/High). This coarse quantization loses the nuanced degrees that make fuzzy logic useful compared to binary thresholds.
- **Fixed membership function shapes**: Only triangular membership functions are provided. Trapezoidal, Gaussian, and sigmoidal shapes — common in control applications — are not supported.
- **Defuzzification is ternary-valued**: The center-of-gravity defuzzification returns a `TernaryMembership`, not a continuous value, which limits downstream precision.
- **No adaptive membership**: Membership function parameters (low/mid/high breakpoints) are fixed at construction time with no online learning or adaptation.

## License

MIT
