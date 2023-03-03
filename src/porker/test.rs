use super::*;

fn create_test_cards(ids: [u32; 5]) -> [Card; 5] {
    let mut cards = [Card {
        id: 0,
        suit: 0,
        rank: 0,
    }; 5];

    cards[0] = Card::new(ids[0]);
    cards[1] = Card::new(ids[1]);
    cards[2] = Card::new(ids[2]);
    cards[3] = Card::new(ids[3]);
    cards[4] = Card::new(ids[4]);

    cards
}

#[test]
fn royalflush() {
    let cards = create_test_cards([0, 9, 10, 11, 12]);
    assert!(is_royalflush(&cards));
    let cards = create_test_cards([0, 9 + 13, 10, 11, 12]);
    assert!(!is_royalflush(&cards));
    let cards = create_test_cards([1, 9, 10, 11, 12]);
    assert!(!is_royalflush(&cards));
    let cards = create_test_cards([0, 2, 10, 11, 13]);
    assert!(!is_royalflush(&cards));
    let cards = create_test_cards([1, 9 + 14, 10, 11, 13]);
    assert!(!is_royalflush(&cards));
}

#[test]
fn straitflush() {
    let mut cards = create_test_cards([0, 1, 2, 3, 4]);
    assert!(is_straitflush(&mut cards));
    let mut cards = create_test_cards([8, 9, 10, 11, 12]);
    assert!(is_straitflush(&mut cards));
    let mut cards = create_test_cards([3 + 13, 4 + 13, 5 + 13, 6 + 13, 7 + 13]);
    assert!(is_straitflush(&mut cards));
    let mut cards = create_test_cards([0, 2, 5, 8, 12]);
    assert!(!is_straitflush(&mut cards));
    let mut cards = create_test_cards([13, 2 + 13, 5 + 13, 8 + 13, 12 + 13]);
    assert!(!is_straitflush(&mut cards));
    let mut cards = create_test_cards([0, 2, 3, 4, 5]);
    assert!(!is_straitflush(&mut cards));
    let mut cards = create_test_cards([0, 14, 2, 3, 4]);
    assert!(!is_straitflush(&mut cards));
}

#[test]
fn fourair() {
    let cards = create_test_cards([0, 13, 26, 39, 4]);
    assert!(is_fourpair(&cards));
    let cards = create_test_cards([5, 13 + 5, 26 + 5, 39 + 5, 4]);
    assert!(is_fourpair(&cards));
    let cards = create_test_cards([9, 13 + 3, 26 + 3, 39 + 3, 3]);
    assert!(is_fourpair(&cards));
    let cards = create_test_cards([0, 13, 25, 39, 4]);
    assert!(!is_fourpair(&cards));
    let cards = create_test_cards([0, 13, 25, 39, 4]);
    assert!(!is_fourpair(&cards));
}

#[test]
fn fulhouse() {
    let mut cards = create_test_cards([0, 13, 26, 4 + 13, 4]);
    assert!(is_fulhouse(&mut cards));
    let mut cards = create_test_cards([0, 13, 4 + 26, 4 + 13, 4]);
    assert!(is_fulhouse(&mut cards));
    let mut cards = create_test_cards([0, 13, 25, 4 + 13, 4]);
    assert!(!is_fulhouse(&mut cards));
}

#[test]
fn flush() {
    let cards = create_test_cards([0, 6, 3, 2, 1]);
    assert!(is_flush(&cards));
    let cards = create_test_cards([0, 14, 3, 2, 1]);
    assert!(!is_flush(&cards));
    let cards = create_test_cards([0, 6, 3, 2, 14]);
    assert!(!is_flush(&cards));
    let cards = create_test_cards([14, 6, 3, 2, 14]);
    assert!(!is_flush(&cards));
}

#[test]
fn strait() {
    let mut cards = create_test_cards([0, 1, 2, 3, 4]);
    assert!(is_strait(&mut cards));
    let mut cards = create_test_cards([0, 9, 10, 11, 12]);
    assert!(is_strait(&mut cards));
    let mut cards = create_test_cards([8, 9, 10, 11, 12]);
    assert!(is_strait(&mut cards));
    let mut cards = create_test_cards([0, 1 + 13, 2 + 26, 3 + 39, 4]);
    assert!(is_strait(&mut cards));
    let mut cards = create_test_cards([8, 9 + 26, 10, 11 + 13, 12 + 39]);
    assert!(is_strait(&mut cards));
    let mut cards = create_test_cards([0, 1, 2, 5 + 13, 4]);
    assert!(!is_strait(&mut cards));
}

#[test]
fn threepair() {
    let cards = create_test_cards([0, 13, 26, 5, 4]);
    assert!(is_threepair(&cards));
    let cards = create_test_cards([1, 14, 27, 5, 4]);
    assert!(is_threepair(&cards));
    let cards = create_test_cards([7, 13, 26, 0, 4]);
    assert!(is_threepair(&cards));
}

#[test]
fn twopair() {
    let cards = create_test_cards([0, 13, 1, 14, 4]);
    assert!(is_twopair(&cards));
    let cards = create_test_cards([9, 13, 0, 14, 1]);
    assert!(is_twopair(&cards));
    let cards = create_test_cards([0, 13, 3, 15, 4]);
    assert!(!is_twopair(&cards));
}

#[test]
fn onepair() {
    let cards = create_test_cards([0, 13, 2, 6, 4]);
    assert!(is_onepair(&cards));
    let cards = create_test_cards([0, 3, 2, 6, 4]);
    assert!(!is_onepair(&cards));
    let cards = create_test_cards([0, 7, 2, 6, 4]);
    assert!(!is_onepair(&cards));
}
