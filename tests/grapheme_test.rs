use talos::render::Grapheme;

#[test]
fn test_grapheme_creation() {
    let g1 = Grapheme::new("a");
    assert_eq!(g1.as_str(), "a");

    let g2 = Grapheme::new("m̀");
    assert_eq!(g2.as_str(), "m̀");

    let g3 = Grapheme::default();
    assert_eq!(g3.as_str(), " ");
}
