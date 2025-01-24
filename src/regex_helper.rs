use regex_syntax::{hir::Hir, ParserBuilder};

///检查正则表达式模式是否包含大写字符。
/// 它通过解析模式到正则表达式的高阶中间表示（HIR），然后递归地分析其结构来完成这一任务。
pub fn pattern_has_uppercase_char(pattern: &str) -> bool {
    let mut parser = ParserBuilder::new().utf8(false).build();

    parser
        .parse(pattern)
        .map(|hir| hir_has_uppercase_char(&hir))
        .unwrap_or(false)
}

fn hir_has_uppercase_char(hir: &Hir) -> bool {
    use regex_syntax::hir::*;

    match hir.kind() {
        HirKind::Literal(Literal(bytes)) => match std::str::from_utf8(bytes) {
            Ok(s) => s.chars().any(|c| c.is_uppercase()),
            Err(_) => bytes.iter().any(|b| char::from(*b).is_uppercase()),
        },
        HirKind::Class(Class::Unicode(ranges)) => ranges
            .iter()
            .any(|r| r.start().is_uppercase() || r.end().is_uppercase()),
        HirKind::Class(Class::Bytes(ranges)) => ranges
            .iter()
            .any(|r| char::from(r.start()).is_uppercase() || char::from(r.end()).is_uppercase()),
        HirKind::Capture(Capture { sub, .. }) | HirKind::Repetition(Repetition { sub, .. }) => {
            hir_has_uppercase_char(sub)
        }
        HirKind::Concat(hirs) | HirKind::Alternation(hirs) => {
            hirs.iter().any(hir_has_uppercase_char)
        }
        _ => false,
    }
}

///检查一个正则表达式模式是否匹配以 . 开头的字符串。
/// 它通过解析正则表达式的高阶中间表示（HIR）来分析模式的结构。
pub fn pattern_matches_strings_with_leading_dot(pattern: &str) -> bool {
    let mut parser = ParserBuilder::new().utf8(false).build();

    parser
        .parse(pattern)
        .map(|hir| hir_matches_strings_with_leading_dot(&hir))
        .unwrap_or(false)
}

fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool {
    use regex_syntax::hir::*;

    match hir.kind() {
        HirKind::Concat(hirs) => {
            let mut hirs = hirs.iter();
            if let Some(hir) = hirs.next() {
                if hir.kind() != &HirKind::Look(Look::Start) {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(hir) = hirs.next() {
                match hir.kind() {
                    HirKind::Literal(Literal(bytes)) => bytes.starts_with(b"."),
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
