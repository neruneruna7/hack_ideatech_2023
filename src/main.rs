//use actix_web::middleware::Logger;
//use actix_web::web::Json;
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//use serde::{Deserialize, Serialize};
use std::time;

mod porker;

/*
//jsonのリクエストのフィールド名と名前が一致するように
//allowアトリビュートで名前がスネークケースでない警告を無視
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Request {
    num: u32,
    useCards: Vec<u32>,
}

#[post["/postcards"]]
async fn judge_porker(request: web::Json<Request>) -> impl Responder {
    //カードをランダムに5枚選び出す（idのみ）
    let handout_id = handout_cards(&request.useCards);
    //idからCard型を生成する
    let mut cards = make_cards_from_id(&handout_id);

    //ループ回数が100万回を超えていたら，100万回まで減らす
    let loop_num = if request.num > 1_000_000 {
        1_000_000
    } else {
        request.num
    };

    for _i in 0..loop_num {}

    HttpResponse::Ok()
}
*/

fn main() {
    let now = time::Instant::now();

    let use_cards = porker::Card::all_cards_id();

    let mut role_count = [0; 10];

    //let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(123);

    for _ in 0..1_000_000 {
        let cards = porker::handout_cards(&use_cards);

        //println!("{:?}", cards);

        let mut cards = porker::make_cards_from_id(&cards);

        //ソート結果を表示
        //cards.sort_by(|a, b| a.rank.cmp(&b.rank));
        //println!("{:?}", cards);

        porker::count_judge_role(&mut cards, &mut role_count);
    }

    println!("{:?}", role_count);

    porker::debug_judge_role(&role_count, 1_000_000);
    println!("{:?}", now.elapsed());
    //仮
}
/*
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:5000");

    HttpServer::new().bind(("127.0.0.1", 5000))?.run().await
}
*/
