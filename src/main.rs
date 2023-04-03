use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum State {
    Named(String),
    Unnamed(usize),
}

struct Transition {
    symbol: char,
    target: State,
}

struct NFA {
    states: BTreeSet<State>,
    initial: State,
    finals: BTreeSet<State>,
    transitions: HashMap<State, Vec<Transition>>,
}

struct DFA {
    states: BTreeSet<State>,
    initial: State,
    finals: BTreeSet<State>,
    transitions: HashMap<State, Vec<Transition>>,
}

fn epsilon_closure(state: &State, nfa: &NFA) -> BTreeSet<State> {
    let mut closure = BTreeSet::new();
    closure.insert(state.clone());

    let mut changed = true;

    while changed {
        changed = false;

        let mut new_states = BTreeSet::new();

        for s in closure.iter() {
            if let Some(transitions) = nfa.transitions.get(s) {
                for t in transitions {
                    if t.symbol == '\0' {
                        if new_states.insert(t.target.clone()) {
                            changed = true;
                        }
                    }
                }
            }
        }

        if changed {
            closure.append(&mut new_states);
        }
    }

    closure
}

fn move_states(states: &BTreeSet<State>, symbol: char, nfa: &NFA) -> BTreeSet<State> {
    let mut next_states = BTreeSet::new();

    for s in states.iter() {
        if let Some(transitions) = nfa.transitions.get(s) {
            for t in transitions {
                if t.symbol == symbol {
                    next_states.insert(t.target.clone());
                }
            }
        }
    }

    next_states
}

fn nfa_to_dfa(nfa: &NFA) -> DFA {
    let mut dfa_states = BTreeSet::new();
    let mut dfa_transitions = HashMap::new();
    let mut dfa_finals = BTreeSet::new();

    let initial_closure = epsilon_closure(&nfa.initial, nfa);
    let initial_state = State::Unnamed(0);

    let mut unnamed_states = vec![initial_closure.clone()];
    let mut named_states = HashMap::new();
    named_states.insert(initial_closure.clone(), initial_state.clone());
    dfa_states.insert(initial_state.clone());

    let mut i = 0;

    while let Some(current_states) = unnamed_states.pop() {
        let current_state = State::Unnamed(i);
        i += 1;

        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            let next_states = move_states(&current_states, c, nfa);

            if !next_states.is_empty() {
                let next_closure = epsilon_closure(&next_states.iter().next().unwrap(), nfa);

                let next_state = if named_states.contains_key(&next_closure) {
                    named_states[&next_closure].clone()
                } else {
                    let state = State::Unnamed(i);
                    i += 1;
                    unnamed_states.push(next_closure.clone());
                    named_states.insert(next_closure.clone(), state.clone());
                    dfa_states.insert(next_state.clone());
                    dfa_transitions.entry(current_state.clone())
                        .or_insert_with(Vec::new)
                        .push(Transition { symbol: c, target: next_state.clone() });
                    next_state
                };

                dfa_transitions.entry(current_state.clone())
                    .or_insert_with(Vec::new)
                    .push(Transition { symbol: c, target: next_state.clone() });
            }
        }

        for nfa_state in &current_states {
        if nfa.finals.contains(nfa_state) {
            dfa_finals.insert(current_state.clone());
            break;
        }
    }
    }

    DFA {
        states: dfa_states,
        initial: initial_state,
        finals: dfa_finals,
        transitions: dfa_transitions,
    }
}

fn main() {
    let re = "a(b|c)*d";
    let nfa = regex::Regex::new(re).unwrap().into_nfa();
    let dfa = nfa_to_dfa(&nfa);

    println!("{:?}", dfa);
}


       