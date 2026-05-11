use pinyin::ToPinyin;

fn is_ascii_alpha_query(query: &str) -> bool {
    !query.is_empty() && query.chars().all(|c| c.is_ascii_alphabetic())
}

pub fn matches_pinyin(text: &str, query: &str) -> bool {
    if !is_ascii_alpha_query(query) {
        return false;
    }

    let mut full_pinyin = String::new();
    let mut initials = String::new();

    for ch in text.chars() {
        if let Some(pinyin) = ch.to_pinyin() {
            let plain = pinyin.plain();
            full_pinyin.push_str(plain);
            initials.push(plain.chars().next().unwrap());
        } else if ch.is_ascii_alphabetic() {
            let lower = ch.to_ascii_lowercase();
            full_pinyin.push(lower);
            initials.push(lower);
        }
    }

    full_pinyin.contains(query) || initials.contains(query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_pinyin_match() {
        assert!(matches_pinyin("北京", "beijing"));
        assert!(matches_pinyin("上海", "shanghai"));
        assert!(matches_pinyin("广州", "guangzhou"));
    }

    #[test]
    fn initial_match() {
        assert!(matches_pinyin("北京", "bj"));
        assert!(matches_pinyin("上海", "sh"));
        assert!(matches_pinyin("广州", "gz"));
    }

    #[test]
    fn partial_pinyin_match() {
        assert!(matches_pinyin("地铁1号线", "ditie"));
        assert!(matches_pinyin("快速公交", "ksgj"));
    }

    #[test]
    fn no_match() {
        assert!(!matches_pinyin("北京", "nanjing"));
        assert!(!matches_pinyin("上海", "bj"));
    }

    #[test]
    fn non_ascii_query_skipped() {
        assert!(!matches_pinyin("北京", "北"));
        assert!(!matches_pinyin("北京", "123"));
    }

    #[test]
    fn mixed_content() {
        assert!(matches_pinyin("K1路", "k"));
        assert!(matches_pinyin("B2路", "b"));
    }
}
