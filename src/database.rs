use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Connection, Row, Sqlite, SqlitePool, Transaction,
};

use crate::{Request, Response};

type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

/// SQLiteのコネクションプールを作成して返す らしい（コネクションプールって何）
pub async fn create_sqlite_pool(database_url: &str) -> DbResult<SqlitePool> {
    // コネクションの設定
    let connection_options = SqliteConnectOptions::from_str(database_url)?
        // DBが存在しないなら作成する
        .create_if_missing(true)
        // トランザクション使用時の性能向上のため、WALを使用する らしい
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    // 上の設定を使ってコネクションプールを作成する
    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(sqlite_pool)
}

pub async fn migrate_database(pool: &SqlitePool) -> DbResult<()> {
    sqlx::migrate!("./db").run(pool).await?;
    Ok(())
}

// ポーカーの実行結果を保存
pub async fn insert_millionpoker(
    pool: &SqlitePool,
    request: &Request,
    response: &Response,
) -> DbResult<()> {
    Ok(())
}

async fn insert_use_cards(
    tx: &mut Transaction<'_, Sqlite>,
    id: u32,
    cards: &Vec<u32>,
    count: u32,
) -> DbResult<()> {
    sqlx::query(
        "
        INSERT INTO used_cards 
        (id, cards, count)
        VALUES 
        (?, ?, ?)
        ON CONFLICT(cards)
        DO UPDATE SET
        (count = count + 1)
    ",
    )
    .bind(id)
    .bind(cards.iter().map(|x| format!("{}", x)).collect::<String>())
    .bind(count)
    .execute(tx)
    .await?;

    Ok(())
}
