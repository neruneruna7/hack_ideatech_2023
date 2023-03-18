-- SQLite
PRAGMA foreign_keys=true;

CREATE TABLE IF NOT EXISTS used_cards (
    id PRIMARY KEY, 
    cards, 
    count
);

CREATE TABLE IF NOT EXISTS role_count (
    id PRIMARY KEY,
    royal_flush,
    strait_flush,
    for_of_a_kind,
    full_house,
    flush,
    strait,
    three_of_a_kind,
    two_pair,
    one_pair,
    no_pair
);

CREATE TABLE IF NOT EXISTS hand (
    id PRIMARY KEY,
    cards_id,
    role_id,
    num,
    sum_score,
    FOREIGN KEY (cards_id) REFERENCES used_cards(id),
    FOREIGN KEY (role_id) REFERENCES role_count(id));
