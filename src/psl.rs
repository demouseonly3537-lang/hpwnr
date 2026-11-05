// Public Suffix List (ICANN section) used to find the registrable label of a host.
// Single-label suffixes like com or uk are covered by the default rule, so only multi-label rules are embedded.
// Each line is an exact suffix, a *.rule wildcard, or a !rule exception.

use std::collections::HashSet;

static PSL_DATA: &str = include_str!("psl.dat");

struct Rules {
    exact: HashSet<&'static str>,
    wildcard: HashSet<&'static str>,
    exception: HashSet<&'static str>,
}

fn rules() -> Rules {
    let mut exact = HashSet::new();
    let mut wildcard = HashSet::new();
    let mut exception = HashSet::new();
    for line in PSL_DATA.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('!') {
            exception.insert(rest);
        } else if let Some(rest) = line.strip_prefix("*.") {
            wildcard.insert(rest);
        } else {
            exact.insert(line);
        }
    }
    Rules { exact, wildcard, exception }
}

// Number of trailing labels that form the public suffix, per the PSL matching algorithm.
fn public_suffix_len(labels: &[&str], r: &Rules) -> usize {
    let n = labels.len();
    // An exception rule wins outright: the suffix is that rule without its leftmost label.
    for i in 0..n {
        let cand = labels[i..].join(".");
        if r.exception.contains(cand.as_str()) {
            return (n - i).saturating_sub(1);
        }
    }
    // Otherwise the longest matching exact or wildcard rule prevails; the default is one label.
    let mut best = 1;
    for i in 0..n {
        let len = n - i;
        let cand = labels[i..].join(".");
        if r.exact.contains(cand.as_str()) && len > best {
            best = len;
        }
        if r.wildcard.contains(cand.as_str()) && len + 1 > best {
            best = len + 1;
        }
    }
    best.min(n)
}

pub fn registrable_domain(host: &str) -> Option<String> {
    let labels: Vec<&str> = host.trim_end_matches('.').split('.').filter(|l| !l.is_empty()).collect();
    if labels.is_empty() {
        return None;
    }
    let r = rules();
    let suffix = public_suffix_len(&labels, &r);
    if suffix >= labels.len() {
        return None;
    }
    Some(labels[labels.len() - suffix - 1..].join("."))
}
