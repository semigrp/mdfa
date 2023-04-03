mDFA (mini-DFA)
This is an implementation of Thompson's algorithm in Rust, which converts a regular expression into a minimal deterministic finite automaton (mDFA). The resulting mDFA can be used to efficiently match strings against the original regular expression.

Usage
To use the mDFA, simply define a regular expression as a string and pass it to the regex::Regex::new() function. This will create an NFA object, which can then be converted to an mDFA using the nfa_to_mdfa() function.

```rust
use regex::Regex;
use mdfa::{nfa_to_mdfa};

fn main() {
    let re = Regex::new(r#"a(b|c)*d"#).unwrap();
    let nfa = re.into_nfa();
    let mdfa = nfa_to_mdfa(&nfa);

    assert!(mdfa.matches("abd"));
    assert!(mdfa.matches("acd"));
    assert!(mdfa.matches("abbbbbbbbd"));
    assert!(!mdfa.matches("ad"));
    assert!(!mdfa.matches("abcd"));
}
```

In this example, the regular expression a(b|c)*d is converted to an mDFA, which is then used to match various strings. The matches() method returns true if the string matches the regular expression, and false otherwise.

License
This project is licensed under the MIT License - see the LICENSE file for details.