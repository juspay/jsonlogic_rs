use std::cmp::{max, Ordering};
use std::collections::VecDeque;
use unicode_normalization::UnicodeNormalization;

fn split_version(version: &str) -> VecDeque<String> {
    let mut components = VecDeque::new();
    let mut part = String::new();
    for ch in version.chars() {
        if ch == '.' || ch == '-' {
            components.push_back(part.drain(..).collect());
            part = String::new();
        } else {
            part.push(ch);
        }
    }
    components.push_back(part.drain(..).collect());
    components
}

pub fn compare_version(str1: &str, str2: &str, alpha_numeric: bool) -> Ordering {
    let norm_str1 = str1.nfc().collect::<String>();
    let norm_str2 = str2.nfc().collect::<String>();
    let mut iter1 = split_version(&norm_str1);
    let mut iter2 = split_version(&norm_str2);

    for _ in 0..max(iter1.len(), iter2.len()) {
        let comp1 = iter1.pop_front();
        let comp2 = iter2.pop_front();

        match (comp1, comp2) {
            (None, None) => return Ordering::Equal,
            (Some(ref _s1), None) => return Ordering::Greater,
            (None, Some(ref _s2)) => return Ordering::Less,
            (Some(s1), Some(s2)) => {
                let order = if alpha_numeric {
                    match (s1.parse::<u32>(), s2.parse::<u32>()) {
                        (Ok(i1), Ok(i2)) => i1.cmp(&i2),
                        _ => s1.cmp(&s2),
                    }
                } else {
                    s1.cmp(&s2)
                };
                match order {
                    Ordering::Equal => continue,
                    _ => return order,
                }
            }
        }
    }

    return Ordering::Equal;
}

#[cfg(test)]
mod tests {
    use crate::operators::jp_version::compare_version;

    #[test]
    fn equal() {
        let mut input = vec![
            "2.5.10.4159",
            "1.0.0",
            "0.5",
            "0.4.1",
            "1",
            "1.1",
            "2.0.4-rc.12",
            "2.0.3-rc.89",
            "2.0.2-rc.12",
            "0.0.0",
            "2.5.0",
            "2.0rc1_460",
            "2",
            "0.0",
            "2.5.10",
            "2.0rc1_300",
            "10.5",
            "1.25.4",
            "1.2.15",
            "2.0.4-rc.31",
            "1.0.4-rc.31",
            "2.1.0-xirctc.01",
            "2.1.0-aplha.01",
            "2.1.0-release.01",
        ];
        let expected = vec![
            "0.0",
            "0.0.0",
            "0.4.1",
            "0.5",
            "1",
            "1.0.0",
            "1.0.4-rc.31",
            "1.1",
            "1.2.15",
            "1.25.4",
            "2",
            "2.0.2-rc.12",
            "2.0.3-rc.89",
            "2.0.4-rc.12",
            "2.0.4-rc.31",
            "2.0rc1_300",
            "2.0rc1_460",
            "2.1.0-aplha.01",
            "2.1.0-release.01",
            "2.1.0-xirctc.01",
            "2.5.0",
            "2.5.10",
            "2.5.10.4159",
            "10.5",
        ];
        input.sort_by(|str1: &&str, str2: &&str| compare_version(*str1, *str2, true));

        assert_eq!(input, expected)
    }
}
