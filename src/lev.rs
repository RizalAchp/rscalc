#[allow(unused)]
pub fn lev(a: impl AsRef<str>, b: impl AsRef<str>) -> usize {
    let a = a.as_ref();
    let b = b.as_ref();
    /* Shortcut optimizations / degenerate cases. */
    if a == b {
        return 0;
    }

    let length_a = a.chars().count();
    let length_b = b.chars().count();
    if length_a == 0 {
        return length_b;
    }
    if length_b == 0 {
        return length_a;
    }

    let mut cache = Vec::from_iter(1usize..=length_a);
    let mut distance_a;
    let mut distance_b;

    let mut result = 0;

    /* Loop. */
    for (index_b, code_b) in b.chars().enumerate() {
        result = index_b;
        distance_a = index_b;

        for (index_a, code_a) in a.chars().enumerate() {
            distance_b = if code_a == code_b {
                distance_a
            } else {
                distance_a + 1
            };

            distance_a = cache[index_a];

            result = if distance_a > result {
                if distance_b > result {
                    result + 1
                } else {
                    distance_b
                }
            } else if distance_b > distance_a {
                distance_a + 1
            } else {
                distance_b
            };

            cache[index_a] = result;
        }
    }

    result
}

#[allow(unused)]
pub fn suggest_word(a: &'_ str, b: &'_ [&'_ str]) -> Vec<(usize, String)> {
    let mut item: Vec<_> = b
        .iter()
        .map(|word| (lev(a, word), word.to_string()))
        .collect();
    item.sort_unstable_by_key(|(k, _)| *k);
    item
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lev() {
        let result = lev("levenshtein", "lefenshdeen");
        assert_eq!(result, 3);

        let result = lev("kitten", "sitten");
        assert_eq!(result, 1);

        let result = lev("kitten", "kitten");
        assert_eq!(result, 0);
    }
}
