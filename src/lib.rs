#![forbid(unsafe_code)]

//! Fuzzy logic with ternary membership.

use std::collections::HashMap;

/// Ternary membership degree: Low (-1), Medium (0), High (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryMembership {
    Low = -1,
    Medium = 0,
    High = 1,
}

impl TernaryMembership {
    pub fn value(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(TernaryMembership::Low),
            0 => Some(TernaryMembership::Medium),
            1 => Some(TernaryMembership::High),
            _ => None,
        }
    }

    pub fn all() -> [TernaryMembership; 3] {
        [TernaryMembership::Low, TernaryMembership::Medium, TernaryMembership::High]
    }
}

/// A ternary fuzzy set with membership function.
#[derive(Debug, Clone)]
pub struct TernaryFuzzySet {
    pub name: String,
    pub memberships: HashMap<String, TernaryMembership>,
}

impl TernaryFuzzySet {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), memberships: HashMap::new() }
    }

    pub fn insert(&mut self, element: &str, membership: TernaryMembership) {
        self.memberships.insert(element.to_string(), membership);
    }

    pub fn membership(&self, element: &str) -> TernaryMembership {
        *self.memberships.get(element).unwrap_or(&TernaryMembership::Low)
    }

    pub fn elements(&self) -> Vec<&str> {
        self.memberships.keys().map(|s| s.as_str()).collect()
    }

    /// Support: elements with non-Low membership.
    pub fn support(&self) -> Vec<&str> {
        self.memberships.iter()
            .filter(|(_, &m)| m != TernaryMembership::Low)
            .map(|(k, _)| k.as_str())
            .collect()
    }

    /// Core: elements with High membership.
    pub fn core(&self) -> Vec<&str> {
        self.memberships.iter()
            .filter(|(_, &m)| m == TernaryMembership::High)
            .map(|(k, _)| k.as_str())
            .collect()
    }
}

/// Membership function mapping a crisp value to ternary membership.
pub trait MembershipFunction {
    fn evaluate(&self, x: f64) -> TernaryMembership;
}

/// Triangular membership function mapping to ternary.
pub struct TriangularTernaryMF {
    pub low: f64,
    pub mid: f64,
    pub high: f64,
}

impl TriangularTernaryMF {
    pub fn new(low: f64, mid: f64, high: f64) -> Self {
        Self { low, mid, high }
    }
}

impl MembershipFunction for TriangularTernaryMF {
    fn evaluate(&self, x: f64) -> TernaryMembership {
        if x <= self.low || x >= self.high {
            TernaryMembership::Low
        } else if (x - self.mid).abs() < f64::EPSILON * 100.0 {
            TernaryMembership::High
        } else if x < self.mid {
            TernaryMembership::Medium
        } else {
            TernaryMembership::Medium
        }
    }
}

/// Step membership function: thresholds for Low/Medium/High.
pub struct StepTernaryMF {
    pub low_threshold: f64,
    pub high_threshold: f64,
}

impl StepTernaryMF {
    pub fn new(low_threshold: f64, high_threshold: f64) -> Self {
        Self { low_threshold, high_threshold }
    }
}

impl MembershipFunction for StepTernaryMF {
    fn evaluate(&self, x: f64) -> TernaryMembership {
        if x < self.low_threshold {
            TernaryMembership::Low
        } else if x < self.high_threshold {
            TernaryMembership::Medium
        } else {
            TernaryMembership::High
        }
    }
}

/// T-norm adapted for ternary (fuzzy AND).
pub fn ternary_tnorm(a: TernaryMembership, b: TernaryMembership) -> TernaryMembership {
    if a.value() <= b.value() { a } else { b }
}

/// T-conorm adapted for ternary (fuzzy OR).
pub fn ternary_tconorm(a: TernaryMembership, b: TernaryMembership) -> TernaryMembership {
    if a.value() >= b.value() { a } else { b }
}

/// Ternary complement (fuzzy NOT).
pub fn ternary_complement(a: TernaryMembership) -> TernaryMembership {
    match a {
        TernaryMembership::Low => TernaryMembership::High,
        TernaryMembership::Medium => TernaryMembership::Medium,
        TernaryMembership::High => TernaryMembership::Low,
    }
}

/// A fuzzy inference rule.
#[derive(Debug, Clone)]
pub struct FuzzyRule {
    pub antecedents: Vec<(String, TernaryMembership)>, // (variable, membership)
    pub operator: RuleOperator,
    pub consequent: (String, TernaryMembership),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleOperator {
    And,
    Or,
}

impl FuzzyRule {
    pub fn new_and(antecedents: Vec<(String, TernaryMembership)>, consequent: (String, TernaryMembership)) -> Self {
        Self { antecedents, operator: RuleOperator::And, consequent }
    }

    pub fn new_or(antecedents: Vec<(String, TernaryMembership)>, consequent: (String, TernaryMembership)) -> Self {
        Self { antecedents, operator: RuleOperator::Or, consequent }
    }

    /// Evaluate the rule's firing strength given current memberships.
    pub fn firing_strength(&self, memberships: &HashMap<String, TernaryMembership>) -> TernaryMembership {
        let values: Vec<TernaryMembership> = self.antecedents.iter().map(|(var, _)| {
            *memberships.get(var).unwrap_or(&TernaryMembership::Low)
        }).collect();

        match self.operator {
            RuleOperator::And => values.iter().fold(TernaryMembership::High, |a, &b| ternary_tnorm(a, b)),
            RuleOperator::Or => values.iter().fold(TernaryMembership::Low, |a, &b| ternary_tconorm(a, b)),
        }
    }
}

/// Defuzzification to a ternary output.
pub fn defuzzify_to_ternary(values: &[TernaryMembership]) -> TernaryMembership {
    if values.is_empty() {
        return TernaryMembership::Medium;
    }
    let sum: i32 = values.iter().map(|v| v.value()).sum();
    let avg = sum as f64 / values.len() as f64;
    if avg > 0.33 {
        TernaryMembership::High
    } else if avg < -0.33 {
        TernaryMembership::Low
    } else {
        TernaryMembership::Medium
    }
}

/// A fuzzy control system.
pub struct FuzzyControlSystem {
    pub input_mfs: HashMap<String, Box<dyn MembershipFunction>>,
    pub rules: Vec<FuzzyRule>,
}

impl FuzzyControlSystem {
    pub fn new() -> Self {
        Self { input_mfs: HashMap::new(), rules: Vec::new() }
    }

    pub fn add_input(&mut self, name: &str, mf: Box<dyn MembershipFunction>) {
        self.input_mfs.insert(name.to_string(), mf);
    }

    pub fn add_rule(&mut self, rule: FuzzyRule) {
        self.rules.push(rule);
    }

    /// Evaluate inputs through the fuzzy system.
    pub fn evaluate(&self, inputs: &HashMap<String, f64>) -> HashMap<String, TernaryMembership> {
        // Fuzzify
        let mut memberships: HashMap<String, TernaryMembership> = HashMap::new();
        for (name, mf) in &self.input_mfs {
            if let Some(&x) = inputs.get(name) {
                memberships.insert(name.clone(), mf.evaluate(x));
            }
        }

        // Apply rules
        let mut outputs: HashMap<String, Vec<TernaryMembership>> = HashMap::new();
        for rule in &self.rules {
            let strength = rule.firing_strength(&memberships);
            let (out_var, out_mem) = &rule.consequent;
            // Combine strength with consequent using t-norm
            let combined = ternary_tnorm(strength, *out_mem);
            outputs.entry(out_var.clone()).or_default().push(combined);
        }

        // Defuzzify
        let mut result = HashMap::new();
        for (var, vals) in outputs {
            result.insert(var, defuzzify_to_ternary(&vals));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_membership_values() {
        assert_eq!(TernaryMembership::Low.value(), -1);
        assert_eq!(TernaryMembership::Medium.value(), 0);
        assert_eq!(TernaryMembership::High.value(), 1);
    }

    #[test]
    fn test_membership_from_i32() {
        assert_eq!(TernaryMembership::from_i32(-1), Some(TernaryMembership::Low));
        assert_eq!(TernaryMembership::from_i32(0), Some(TernaryMembership::Medium));
        assert_eq!(TernaryMembership::from_i32(1), Some(TernaryMembership::High));
        assert_eq!(TernaryMembership::from_i32(2), None);
    }

    #[test]
    fn test_fuzzy_set_basic() {
        let mut fs = TernaryFuzzySet::new("temperature");
        fs.insert("cold", TernaryMembership::High);
        fs.insert("warm", TernaryMembership::Medium);
        fs.insert("hot", TernaryMembership::Low);
        assert_eq!(fs.membership("cold"), TernaryMembership::High);
        assert_eq!(fs.membership("nonexistent"), TernaryMembership::Low);
    }

    #[test]
    fn test_fuzzy_set_support() {
        let mut fs = TernaryFuzzySet::new("test");
        fs.insert("a", TernaryMembership::High);
        fs.insert("b", TernaryMembership::Medium);
        fs.insert("c", TernaryMembership::Low);
        let support = fs.support();
        assert!(support.contains(&"a"));
        assert!(support.contains(&"b"));
        assert!(!support.contains(&"c"));
    }

    #[test]
    fn test_fuzzy_set_core() {
        let mut fs = TernaryFuzzySet::new("test");
        fs.insert("a", TernaryMembership::High);
        fs.insert("b", TernaryMembership::Medium);
        let core = fs.core();
        assert!(core.contains(&"a"));
        assert!(!core.contains(&"b"));
    }

    #[test]
    fn test_triangular_mf() {
        let mf = TriangularTernaryMF::new(0.0, 50.0, 100.0);
        assert_eq!(mf.evaluate(-1.0), TernaryMembership::Low);
        assert_eq!(mf.evaluate(50.0), TernaryMembership::High);
        assert_eq!(mf.evaluate(25.0), TernaryMembership::Medium);
        assert_eq!(mf.evaluate(101.0), TernaryMembership::Low);
    }

    #[test]
    fn test_step_mf() {
        let mf = StepTernaryMF::new(30.0, 70.0);
        assert_eq!(mf.evaluate(20.0), TernaryMembership::Low);
        assert_eq!(mf.evaluate(50.0), TernaryMembership::Medium);
        assert_eq!(mf.evaluate(80.0), TernaryMembership::High);
    }

    #[test]
    fn test_tnorm() {
        assert_eq!(ternary_tnorm(TernaryMembership::High, TernaryMembership::Low), TernaryMembership::Low);
        assert_eq!(ternary_tnorm(TernaryMembership::High, TernaryMembership::High), TernaryMembership::High);
        assert_eq!(ternary_tnorm(TernaryMembership::Medium, TernaryMembership::High), TernaryMembership::Medium);
    }

    #[test]
    fn test_tconorm() {
        assert_eq!(ternary_tconorm(TernaryMembership::High, TernaryMembership::Low), TernaryMembership::High);
        assert_eq!(ternary_tconorm(TernaryMembership::Low, TernaryMembership::Low), TernaryMembership::Low);
    }

    #[test]
    fn test_complement() {
        assert_eq!(ternary_complement(TernaryMembership::Low), TernaryMembership::High);
        assert_eq!(ternary_complement(TernaryMembership::High), TernaryMembership::Low);
        assert_eq!(ternary_complement(TernaryMembership::Medium), TernaryMembership::Medium);
    }

    #[test]
    fn test_rule_firing_and() {
        let rule = FuzzyRule::new_and(
            vec![("temp".to_string(), TernaryMembership::High), ("humidity".to_string(), TernaryMembership::High)],
            ("fan".to_string(), TernaryMembership::High),
        );
        let mut mems = HashMap::new();
        mems.insert("temp".to_string(), TernaryMembership::High);
        mems.insert("humidity".to_string(), TernaryMembership::High);
        assert_eq!(rule.firing_strength(&mems), TernaryMembership::High);
    }

    #[test]
    fn test_rule_firing_or() {
        let rule = FuzzyRule::new_or(
            vec![("a".to_string(), TernaryMembership::High), ("b".to_string(), TernaryMembership::Low)],
            ("out".to_string(), TernaryMembership::High),
        );
        let mut mems = HashMap::new();
        mems.insert("a".to_string(), TernaryMembership::High);
        mems.insert("b".to_string(), TernaryMembership::Low);
        assert_eq!(rule.firing_strength(&mems), TernaryMembership::High);
    }

    #[test]
    fn test_defuzzify_high() {
        let vals = vec![TernaryMembership::High, TernaryMembership::High, TernaryMembership::High];
        assert_eq!(defuzzify_to_ternary(&vals), TernaryMembership::High);
    }

    #[test]
    fn test_defuzzify_mixed() {
        let vals = vec![TernaryMembership::High, TernaryMembership::Low];
        assert_eq!(defuzzify_to_ternary(&vals), TernaryMembership::Medium);
    }

    #[test]
    fn test_defuzzify_empty() {
        assert_eq!(defuzzify_to_ternary(&[]), TernaryMembership::Medium);
    }

    #[test]
    fn test_fuzzy_control_system() {
        let mut sys = FuzzyControlSystem::new();
        sys.add_input("temp", Box::new(StepTernaryMF::new(30.0, 70.0)));
        sys.add_rule(FuzzyRule::new_and(
            vec![("temp".to_string(), TernaryMembership::High)],
            ("fan".to_string(), TernaryMembership::High),
        ));
        let mut inputs = HashMap::new();
        inputs.insert("temp".to_string(), 80.0);
        let result = sys.evaluate(&inputs);
        assert_eq!(result.get("fan"), Some(&TernaryMembership::High));
    }

    #[test]
    fn test_fuzzy_control_multiple_rules() {
        let mut sys = FuzzyControlSystem::new();
        sys.add_input("temp", Box::new(StepTernaryMF::new(30.0, 70.0)));
        sys.add_input("humidity", Box::new(StepTernaryMF::new(40.0, 80.0)));
        sys.add_rule(FuzzyRule::new_and(
            vec![("temp".to_string(), TernaryMembership::High), ("humidity".to_string(), TernaryMembership::High)],
            ("fan".to_string(), TernaryMembership::High),
        ));
        sys.add_rule(FuzzyRule::new_and(
            vec![("temp".to_string(), TernaryMembership::Low)],
            ("fan".to_string(), TernaryMembership::Low),
        ));
        let mut inputs = HashMap::new();
        inputs.insert("temp".to_string(), 20.0);
        inputs.insert("humidity".to_string(), 90.0);
        let result = sys.evaluate(&inputs);
        // temp=Low triggers low rule, humidity=High but AND with temp=Low → Low
        assert_eq!(result.get("fan"), Some(&TernaryMembership::Low));
    }

    #[test]
    fn test_defuzzify_low() {
        let vals = vec![TernaryMembership::Low, TernaryMembership::Low, TernaryMembership::Low];
        assert_eq!(defuzzify_to_ternary(&vals), TernaryMembership::Low);
    }

    #[test]
    fn test_rule_missing_input() {
        let rule = FuzzyRule::new_and(
            vec![("missing".to_string(), TernaryMembership::High)],
            ("out".to_string(), TernaryMembership::High),
        );
        let mems = HashMap::new();
        // Missing input defaults to Low
        assert_eq!(rule.firing_strength(&mems), TernaryMembership::Low);
    }

    #[test]
    fn test_fuzzy_set_elements() {
        let mut fs = TernaryFuzzySet::new("test");
        fs.insert("a", TernaryMembership::High);
        fs.insert("b", TernaryMembership::Medium);
        assert_eq!(fs.elements().len(), 2);
    }
}
