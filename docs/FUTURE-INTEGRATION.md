# Future Integration: ternary-fuzzy

## Current State
Provides fuzzy logic with ternary membership: `TernaryFuzzySet` with `TernaryMembership` degrees (Low/Medium/High), fuzzy inference rules with `TernaryOperator` (And/Or/Not), and defuzzification to crisp ternary values. Handles uncertainty by allowing partial membership rather than hard boundaries.

## Integration Opportunities

### With ternary-cell (Fuzzy Room State)
ternary-cell's vibe phase produces a 16-dimensional state vector with crisp ternary values. ternary-fuzzy softens these to fuzzy membership degrees: instead of a cell being definitively "positive" (+1), it's "70% High, 30% Medium, 0% Low." This captures uncertainty in cell state — a cell that's barely positive (energy just above threshold) has partial membership. Fuzzy inference rules combine multiple cell signals into room-level assessments: IF energy IS High AND surprise IS Low THEN room_state IS Stable.

### With ternary-memory (Fuzzy Retrieval)
ternary-memory's `MemoryIndex` uses exact context tag matching. ternary-fuzzy enables fuzzy retrieval: memories are indexed with `TernaryMembership` tags, and queries use fuzzy matching. A query for "High surprise" also returns "Medium surprise" memories with reduced weight. `TernaryFuzzySet::membership()` provides the matching function, and defuzzification selects the best match.

### With ternary-scheduling (Fuzzy Priority)
ternary-scheduling uses crisp `TernaryDecision` (Prioritize/Defer/Neutral). ternary-fuzzy enables fuzzy priority: a task can be "mostly Prioritize with some Neutral" when urgency is ambiguous. Fuzzy inference rules combine multiple scheduling signals (deadline proximity, resource availability, task importance) into a fuzzy priority that defuzzifies to a crisp scheduling decision.

## Potential in Mature Systems
In room-as-codespace, rooms operate under uncertainty — sensor readings are noisy, user intent is ambiguous, resource availability fluctuates. ternary-fuzzy provides the reasoning layer that handles this uncertainty gracefully. PLATO's room state assessment uses fuzzy inference: a room is "mostly healthy with some concern about latency" rather than a binary healthy/unhealthy. This enables nuanced resource allocation: rooms that are "fuzzy borderline" get more monitoring but not full intervention.

## Cross-Pollination Ideas
- **ternary-pareto**: Fuzzy Pareto dominance — soften the dominance relation with fuzzy membership, enabling "approximately Pareto-optimal" solutions when exact Pareto front is hard to compute.
- **ternary-kalman**: Fuzzy Kalman filtering — use fuzzy membership to model non-Gaussian uncertainty in state estimates, combining Kalman precision with fuzzy flexibility.
- **ternary-lattice**: Fuzzy lattice — extend ternary-lattice's partial order with fuzzy degrees, where join/meet operations preserve membership values.

## Dependencies for Next Steps
- Define `FuzzyRoomState` with `TernaryMembership` degrees for each state dimension
- Add fuzzy retrieval to ternary-memory's `MemoryIndex`
- Implement fuzzy inference rules for PLATO room health assessment
- Benchmark fuzzy operations on ESP32 (membership lookup table in Flash)
