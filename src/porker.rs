//! ポーカーモジュール

//use rand::prelude::*;
//use rand::rngs::SmallRng;
use rand::{thread_rng, Rng};
use rustc_hash::FxHashMap;
use std::convert::TryInto;

#[cfg(test)]
mod test;

///カード1枚のデータを保持する構造体です．ID,スート(記号), ランク(数字)からできています．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub id: u32,
    pub suit: u32,
    pub rank: u32,
}

impl Card {
    ///IDを渡すことで，スートとランクを計算し，Card型を生成します．
    pub fn new<T>(id: T) -> Card
    where
        T: TryInto<u32>,
        <T as std::convert::TryInto<u32>>::Error: std::fmt::Debug,
    {
        let id: u32 = id.try_into().unwrap();
        let suit = id / 13;
        let rank = (id % 13) + 1;
        Card { id, suit, rank }
    }

    ///デバッグ用に52枚すべてのカードidをもったベクタを返します．
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
        suit: 0,
        rank: 0,
    }; 5];

    for (i, v) in cards_id.iter().enumerate() {
        cards[i] = Card::new(*v);
    }

    cards
}

/// 使用するカードのID一覧を持つベクタから，ランダムに選んだ5枚で手札ID配列を生成します
pub fn handout_cards(use_cards: &Vec<u32>) -> [u32; 5] {
    //ハッシュマップに突っ込むx5した方が早いかも？
    //重複回避のためのハッシュマップ

    //let mut handout_hash: HashMap<u32, usize> = HashMap::new();
    let mut handout_hash = FxHashMap::default();
    let mut rng = thread_rng();

    //entryを使って重複を排除
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


pub fn is_pair(cards: &[Card; 5], pair_num: u32) -> bool {
    //同じランクのカードがpair_num個あるかどうか
    for i in 0..5 {
        let mut count = 1;

        for j in i + 1..5 {
            if cards[i].rank == cards[j].rank {
                count += 1;
            }
            if count == pair_num {
                return true;
            }
        }
    }

    false
}

pub fn is_flush(cards: &[Card; 5]) -> bool {
    //すべてのスートが同じかどうか
    let mut is_flush = true;

    for i in 0..4 {
        if cards[i].suit != cards[i + 1].suit {
            is_flush = false;
        }
    }

    is_flush
}

pub fn is_strait(cards: &mut [Card; 5]) -> bool {
    cards.sort_unstable_by(|a, b| a.rank.cmp(&b.rank));

    let is_strait_1 = cards[0].rank == 1
        && cards[1].rank == 10
        && cards[2].rank == 11
        && cards[3].rank == 12
        && cards[4].rank == 13;

    let mut is_strait_2 = true;
    for i in 0..4 {
        if cards[i].rank + 1 != cards[i + 1].rank {
            is_strait_2 = false;
        }
    }

    is_strait_1 || is_strait_2
}

pub fn is_royalflush(cards: &[Card; 5]) -> bool {
    if !is_flush(cards) {
        return false;
    }

    let mut is_royalflush = true;
    for i in cards.iter().take(5) {
        if i.rank < 10 && i.rank > 1 {
            is_royalflush = false;
        }
    }

    is_royalflush
}

pub fn is_straitflush(cards: &mut [Card; 5]) -> bool {
    is_strait(cards) && is_flush(cards)
}

pub fn is_twopair(cards: &[Card; 5]) -> bool {
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
        return false;
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

    count1 && count2
}

pub fn is_fulhouse(cards: &mut [Card; 5]) -> bool {
    cards.sort_unstable_by(|a, b| a.rank.cmp(&b.rank));

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

    is_fulhouse_1 || is_fulhouse_2
}

pub fn is_onepair(cards: &[Card; 5]) -> bool {
    is_pair(cards, 2)
}

pub fn is_threepair(cards: &[Card; 5]) -> bool {
    is_pair(cards, 3)
}

pub fn is_fourpair(cards: &[Card; 5]) -> bool {
    is_pair(cards, 4)
}

/// 役判定を行います.
pub fn count_judge_role(cards: &mut [Card; 5], role_count: &mut [u32; 10]) {
    if is_royalflush(cards) {
        role_count[9] += 1;
    } else if is_straitflush(cards) {
        role_count[8] += 1;
    } else if is_strait(cards) {
        role_count[7] += 1;
    } else if is_flush(cards) {
        role_count[6] += 1;
    } else if is_fulhouse(cards) {
        role_count[5] += 1;
    } else if is_fourpair(cards) {
        role_count[4] += 1;
    } else if is_threepair(cards) {
        role_count[3] += 1;
    } else if is_twopair(cards) {
        role_count[2] += 1;
    } else if is_onepair(cards) {
        role_count[1] += 1;
    } else {
        role_count[0] += 1;
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
    let score_sheet = [1, 5, 10, 20, 100, 150, 200, 500, 800, 1500];

    let sum_score:u32 = role_count.iter()
        .zip(score_sheet.iter())
        .map(|x| x.0 * x.1)
        .sum();

    sum_score
}
