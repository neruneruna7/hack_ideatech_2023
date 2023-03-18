//! ポーカーモジュール 個々の役を判定する関数は，事前にrankの順にソートされていることを前提としています．

//use rand::prelude::*;
//use rand::rngs::SmallRng;
use rand::{thread_rng, Rng};
use rustc_hash::FxHashMap;
use std::{convert::TryInto};
use num_derive::FromPrimitive;


//mod test;

///カード1枚のデータを保持する構造体です．ID,スート(記号), ランク(数字)からできています．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Card {
    pub id: u32,
    pub suit: Suit,
    pub rank: u32,
}

/// 記号情報を保持する列挙型です． NumクレートのFromPrimitivを活用することにより，u32型をSuit型に変換する機能を提供しています．
/// num::FromPrimitive::from_u32(<u32>).unwrap() でu32型からSuit型に変換できます．
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(u32)]
pub enum Suit {
    #[default]
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(PartialEq, Eq)]
pub enum Role {
    NoPair,
    OnePair,
    TwoPair,
    ThreeCard,
    Straight,
    Flush,
    FullHouse,
    FourCard,
    StraightFlush,
    RoyalStraightFlush,
}

impl Card {
    ///IDを渡すことで，スートとランクを計算し，Card型を生成します．
    pub fn new<T>(id: T) -> Self
    where
        T: TryInto<u32>,
        <T as std::convert::TryInto<u32>>::Error: std::fmt::Debug,
    {
        let id = id.try_into().unwrap();
        let suit = num::FromPrimitive::from_u32(id / 13).unwrap();
        let rank = (id % 13) + 1;
        Self { id, suit, rank}
    }

    /// デバッグ用に52枚すべてのカードidをもったベクタを返します．
    #[allow(unused)]
    pub fn all_cards_id() -> Vec<u32> {
        let mut cards = Vec::new();

        for i in 0..52 {
            cards.push(i);
        }

        cards
    }
}



///手札のカードのid配列を読み込んでCard型配列に変換します．
pub fn make_cards_from_id(cards_id: &[u32; 5]) -> [Card; 5] {
    let mut cards = [Card {
        id: 0,
        suit: num::FromPrimitive::from_u32(0).unwrap(),
        rank: 0,
    }; 5];

    for (i, v) in cards_id.iter().enumerate() {
        cards[i] = Card::new(*v);
    }

    cards
}

/// 使用するカードのID一覧を持つベクタから，ランダムに選んだ5枚で手札ID配列を生成します
pub fn handout_cards(use_cards: &Vec<u32>) -> [u32; 5] {
    // 重複回避のためにハッシュマップを使用しています

    let mut handout_hash = FxHashMap::default();
    let mut rng = thread_rng();

    // entryを使って重複をしないようにデータを挿入
    while handout_hash.iter().len() < 5 {
        let len = handout_hash.len();
        let num = rng.gen_range(0..use_cards.len());

        //entryを使うのとif let 使うの あまり速度変わらなかった
        //見やすいentryを実行している
        handout_hash.entry(use_cards[num]).or_insert(len);
        /*
        if let None = handout_hash.get(&use_cards[num]){
            handout_hash.insert(use_cards[num], len);
        }
        */
    }

    //5枚だとわかっているので，ハッシュマップの値を配列に変換
    let mut handout_id: [u32; 5] = [0; 5];

    for (key, value) in handout_hash.iter() {
        handout_id[*value] = *key;
    }

    handout_id
}


/// 同じランクのカードが何枚あるかを数え，その枚数に応じたRoleを返します．
/// Roleを返せない場合，明確にエラーである（同じランクのカードが5枚以上あることになる）ため，Result型でエラーを返します．
pub fn is_pair(cards: &[Card; 5]) -> Result<Role, &str> {
    // 同じランクのカードが何枚あるかを数える．
    // 大幅な仕様変更がありました．
    let pair_count = (0..5)
        .map(|i| cards.iter().filter(|x| x.rank == cards[i].rank).count())
        .max()
        .unwrap();
    
    // pair_countが2の場合はツーペアの可能性があるため，処理を分岐しています
    match pair_count {
        4 => Ok(Role::FourCard),
        3 => Ok(Role::ThreeCard),
        2 => {
            if is_twopair(cards).is_some() {
                return Ok(Role::TwoPair);
            }else{
                return Ok(Role::OnePair);
            }
        },
        1 => Ok(Role::NoPair),
        _ => Err("Error: invalid pair_num 同じランクのカードの枚数が5枚以上ではありませんか？"),
    }
}

pub fn is_flush(cards: &[Card; 5]) -> Option<Role> {
    // すべてのスートが同じかどうか
    if cards.iter().all(|x| x.suit == cards[0].suit) {
        return Some(Role::Flush);
    }else{
        return None;
    }
}

pub fn is_strait(cards: &mut [Card; 5]) -> Option<Role> {
    // すべてのランクが連続しているかどうか
    
    // エースハイストレートの場合は，1, 10, 11, 12, 13となる．
    const ACE_HIGH: [u32; 5] = [1, 10, 11, 12, 13];

    let is_strait_1 = cards.iter().zip(ACE_HIGH.iter()).all(|(a, b)| a.rank == *b);
    let is_strait_2 = (0..4).all(|i| cards[i].rank + 1 == cards[i + 1].rank);

    if is_strait_1 || is_strait_2 {
        return Some(Role::Straight);
    } else {
        return None;
    }
}

pub fn is_royalflush(cards: &[Card; 5]) -> Option<Role> {
    if is_flush(cards).is_none() {
        return None;
    }

    let mut is_royalflush = Some(Role::RoyalStraightFlush);
    for i in cards.iter().take(5) {
        if i.rank < 10 && i.rank > 1 {
            is_royalflush = None;
        }
    }

    is_royalflush
}

pub fn is_straitflush(cards: &mut [Card; 5]) -> Option<Role> {
    if is_strait(cards).is_some() && is_flush(cards).is_some() {
        return Some(Role::StraightFlush);
    } else {
        return None;
    }
}

pub fn is_twopair(cards: &[Card; 5]) -> Option<Role> {
    let mut count1 = false;
    let mut counted = 999;

    'outer: for i in 0..5 {
        counted = cards[i].rank;

        for j in i + 1..5 {
            if cards[i].rank == cards[j].rank {
                count1 = true;
                break 'outer;
            }
        }
    }

    if !count1 {
        return None;
    }

    let mut count2 = false;
    'outer: for i in 0..5 {
        if cards[i].rank == counted {
            continue;
        }

        for j in i + 1..5 {
            if cards[i].rank == cards[j].rank {
                count2 = true;
                break 'outer;
            }
        }
    }

    if count1 && count2{
        return Some(Role::TwoPair);
    }else {
        return None;
    }
}

pub fn is_fulhouse(cards: &mut [Card; 5]) -> Option<Role> {
    //  rankをキーにソートされているならば2パターンしかありません

    let is_fulhouse_1 = 
        //cards[0].rank == cards[0].rank&&
        cards[1].rank == cards[0].rank
        //&& cards[2].rank == cards[2].rank
        && cards[3].rank == cards[2].rank
        && cards[4].rank == cards[2].rank
    ;

    let is_fulhouse_2 = 
        //cards[0].rank == cards[0].rank&&
        cards[1].rank == cards[0].rank
        && cards[2].rank == cards[0].rank
        //&& cards[3].rank == cards[3].rank
        && cards[4].rank == cards[3].rank
    ;

    if is_fulhouse_1 || is_fulhouse_2{
        return Some(Role::FullHouse);
    }else{
        return None;
    }
}

/// 役判定を行います.
pub fn count_judge_role(cards: &mut [Card; 5], role_count: &mut [u32; 10]) {
    // 事前にカード配列をソートしておく
    // カード配列をrankをキーにソート． 安定ソートである必要はないため，unstable で不安定ソートを使うことにより高速化
    cards.sort_unstable_by(|a, b| a.rank.cmp(&b.rank));

    if is_royalflush(cards).is_some() {
        role_count[9] += 1;
    } else if is_straitflush(cards).is_some() {
        role_count[8] += 1;
    } else if is_strait(cards).is_some() {
        role_count[7] += 1;
    } else if is_flush(cards).is_some() {
        role_count[6] += 1;
    } else if is_fulhouse(cards).is_some() {
        role_count[5] += 1;
    } else if let Ok(r) = is_pair(cards) {
        match r {
            Role::FourCard => role_count[4] += 1,
            Role::ThreeCard => role_count[3] += 1,
            Role::TwoPair => role_count[2] += 1,
            Role::OnePair => role_count[1] += 1,
            _ =>         role_count[0] += 1,
        }
    } 
}

/// デバッグ用に，それぞれの役が出る確率を計算して表示します．
pub fn debug_judge_role(role_count: &[u32; 10], total_num_of_atempt: u32) {
    let roles = [
        "ノーペア",
        "ワンペア",
        "ツーペア",
        "スリーカード",
        "フォーカード",
        "フルハウス",
        "フラッシュ",
        "ストレート",
        "ストレートフラッシュ",
        "ロイヤルストレートフラッシュ",
    ];
    let mut rate = [0.; 10];

    for i in 0..10 {
        rate[i] = role_count[i] as f64 / total_num_of_atempt as f64;
        println!("{:<20}: {:.5}%", roles[i], rate[i] * 100.);
    }
    println!();
}

/// 必要な処理がひとまとめになった関数です．
/// 回数制限，手札選び，役判定，指定回数ループ，スコア計算
/// 事実上，pubキーワードはこの関数にのみついていれば問題ありません．
pub fn million_porker<T>(use_cards: &Vec<u32>, num: T) -> ([u32;10], u32, u32)
where
    T: TryInto<u32>,
    <T as std::convert::TryInto<u32>>::Error: std::fmt::Debug,
{   
    let num:u32 = num.try_into().expect("ERR 回数を整数に変換できません");

    //ループ回数が100万回を超えていたら，100万回まで減らす
    let loop_num = if num > 1_000_000 {
        1_000_000
    } else {
        num
    };

    let mut role_count= [0; 10] ;

    for _ in 0..loop_num{
        //カードをランダムに5枚選び出す（idのみ）
        let cards = handout_cards(use_cards);
        //idからCard型を生成する
        let mut cards = make_cards_from_id(&cards);

        count_judge_role(&mut cards, &mut role_count);
    }

    let sum_score = calc_score(&role_count);
    
    (role_count, sum_score, loop_num)
}

/// 総スコアを計算します．
pub fn calc_score(role_count: &[u32;10]) -> u32 {
    /*indexの小さい順に
        ノーペア,
        ワンペア,
        ツーペア,
        スリーカード,
        フォーカード,
        フルハウス,
        フラッシュ,
        ストレート,
        ストレートフラッシュ,
        ロイヤルストレートフラッシュ, 
    */
    const SCRE_SHEET:[u32;10] = [1, 5, 10, 20, 100, 150, 200, 500, 800, 1500];

    let sum_score:u32 = role_count.iter()
        .zip(SCRE_SHEET.iter())
        .map(|x| x.0 * x.1)
        .sum();

    sum_score

}
