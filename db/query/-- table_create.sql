-- SQLite
CREATE TABLE used_cards (id PRIMARY KEY, cards, count);

CREATE TABLE role_count (
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
    no_pair);


CREATE TABLE hand (
    id PRIMARY KEY,
    cards_id,
    role_id,
    num,
    sum_score,
    FOREIGN KEY (cards_id) REFERENCES used_cards(id),
    FOREIGN KEY (role_id) REFERENCES role_count(id));
