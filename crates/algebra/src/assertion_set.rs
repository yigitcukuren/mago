use mago_codex::assertion::Assertion;

/// A type alias representing a disjunction (an "OR" clause) of items.
///
/// For example, `Or<Assertion>` is equivalent to `(Assertion1 OR Assertion2 OR ...)`.
pub type Or<T> = Vec<T>;

/// A type alias representing a conjunction (an "AND" clause) of items.
///
/// For example, `And<Clause>` is equivalent to `(Clause1 AND Clause2 AND ...)`.
pub type And<T> = Vec<T>;

/// Represents a logical formula in Conjunctive Normal Form (CNF).
///
/// Each inner `Vec<Assertion>` is a single "OR" clause (a disjunction),
/// and the outer `Vec` represents an "AND" of all these clauses (a conjunction).
///
/// For example, `vec![vec![A, B], vec![C]]` corresponds to the logical
/// formula `(A OR B) AND (C)`.
///
/// See: [Conjunctive Normal Form](https://en.wikipedia.org/wiki/Conjunctive_normal_form)
pub type AssertionSet = And<Or<Assertion>>;

/// Applies an `OR` operation to a formula in Conjunctive Normal Form (CNF).
///
/// This function takes a single `Assertion` and adds it to every existing `OR`
/// clause within the formula. For example, applying `C` to `(A) AND (B)`
/// results in `(A OR C) AND (B OR C)`.
///
/// See: [Distributive property](https://en.wikipedia.org/wiki/Distributive_property)
pub fn add_or_assertion(possibilities: &mut AssertionSet, assertion: Assertion) {
    if possibilities.is_empty() {
        // If the formula was empty (representing `true`), the result
        // is a single clause with the new assertion.
        possibilities.push(vec![assertion]);
        return;
    }

    for clause in possibilities {
        clause.push(assertion.clone());
    }
}

/// Applies an `AND` operation to a formula in Conjunctive Normal Form (CNF).
///
/// This function takes a single `Assertion` and adds it as a new, separate `AND`
/// clause to the formula. For example, applying `C` to `(A OR B)`
/// results in `(A OR B) AND (C)`.
pub fn add_and_assertion(possibilities: &mut AssertionSet, assertion: Assertion) {
    // Add a new clause containing only the new assertion.
    possibilities.push(vec![assertion]);
}

/// Applies an `AND` operation with a new `OR` clause to a CNF formula.
///
/// This function adds a new clause, which is itself a disjunction of the
/// provided assertions. For example, applying `(C OR D)` to `(A OR B)`
/// results in `(A OR B) AND (C OR D)`.
pub fn add_and_clause(possibilities: &mut Vec<Vec<Assertion>>, or_assertions: &[Assertion]) {
    if or_assertions.is_empty() {
        // An empty OR clause is equivalent to `false`. ANDing with `false`
        // makes the entire formula `false`, represented by a single empty clause.
        *possibilities = vec![vec![]];
        return;
    }

    possibilities.push(or_assertions.to_vec());
}

/// Negates a formula in Conjunctive Normal Form (CNF).
///
/// This function applies De Morgan's laws to the formula. The process involves:
/// 1. Converting the CNF formula `(A OR B) AND C` to its negated DNF form: `(NOT A AND NOT B) OR (NOT C)`.
/// 2. Converting the resulting DNF back to CNF using the distributive property.
pub fn negate_assertion_set(assertion_set: AssertionSet) -> AssertionSet {
    // 1. Apply De Morgan's laws to get the DNF representation.
    //    `(A OR B) AND C` becomes `(¬A AND ¬B) OR (¬C)`.
    let dnf: AssertionSet = assertion_set
        .into_iter()
        .map(|or_clause| or_clause.into_iter().map(|a| a.get_negation()).collect::<Vec<_>>())
        .filter(|and_clause| !and_clause.is_empty())
        .collect();

    if dnf.is_empty() {
        // The original formula was `true` (no clauses), so its negation is `false`.
        // A `false` CNF is represented by a single, empty OR clause.
        return vec![vec![]];
    }

    // 2. Convert the DNF back to CNF.
    //    Start with the first AND clause of the DNF, converted to CNF.
    //    e.g., `(¬A AND ¬B)` becomes `(¬A) AND (¬B)`.
    let mut result_cnf: AssertionSet = dnf[0].iter().map(|literal| vec![literal.clone()]).collect();

    // Iteratively combine the rest of the DNF clauses.
    for and_clause in dnf.iter().skip(1) {
        let mut next_result_cnf = AssertionSet::new();
        // This is the distributive step: (F1) OR (A AND B) => (F1 OR A) AND (F1 OR B)
        // where F1 is the current CNF.
        for literal in and_clause {
            for cnf_clause in &result_cnf {
                let mut new_clause = cnf_clause.clone();
                new_clause.push(literal.clone());
                next_result_cnf.push(new_clause);
            }
        }

        result_cnf = next_result_cnf;
    }

    result_cnf
}
