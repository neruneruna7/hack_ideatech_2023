use std::result;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
//use std::time;

mod porker;

//jsonのリクエストのフィールド名と名前が一致するように
//allowアトリビュートで名前がスネークケースでない警告を無視
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Request {
    num: u32,
    useCards: Vec<u32>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct ResultRole {
    nopair: u32,
    onepair: u32,
    twopair: u32,
    threepair: u32,
    fourpair: u32,
    fulhouse: u32,
    flush: u32,
    strait: u32,
    straitflush: u32,
    royalflush: u32,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct Response {
    allscore: u32,
    number: u32,
    result: ResultRole,
}

impl Response {
    fn new(all_score: u32, number: u32, role_count: [u32; 10]) -> Response {
        Response {
            allscore: all_score,
            number,
            result: ResultRole {
                nopair: role_count[0],
                onepair: role_count[1],
                twopair: role_count[2],
                threepair: role_count[3],
                fourpair: role_count[4],
                fulhouse: role_count[5],
                flush: role_count[6],
                strait: role_count[7],
                straitflush: role_count[8],
                royalflush: role_count[9],
            },
        }
    }
}

#[post["/postcards"]]
async fn judge_porker(request: web::Json<Request>) -> impl Responder {
    //ループ回数が100万回を超えていたら，100万回まで減らす
    let loop_num = if request.num > 1_000_000 {
        1_000_000
    } else {
        request.num
    };

    let mut role_count = [0; 10];

    porker::million_porker(&request.useCards, &mut role_count, loop_num);

    porker::debug_judge_role(&role_count, loop_num);

    HttpResponse::Ok().json(Response::new(1000, loop_num, role_count))
}

#[get["/"]]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get["/Una"]]
async fn una() -> impl Responder {
    HttpResponse::Unauthorized().body("401 Unauthrized")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:5000");

    HttpServer::new(|| {
        App::new()
            .service(get_index)
            .service(judge_porker)
            .service(una)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

/*
fn main() {
    let now = time::Instant::now();

    let use_cards = porker::Card::all_cards_id();

    let mut role_count = [0; 10];

    let num = 1_000_000;

    //let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(123);

    /*
    for _ in 0..1_000_000 {
        let cards = porker::handout_cards(&use_cards);

        //println!("{:?}", cards);

        let mut cards = porker::make_cards_from_id(&cards);

        //ソート結果を表示
        //cards.sort_by(|a, b| a.rank.cmp(&b.rank));
        //println!("{:?}", cards);

        porker::count_judge_role(&mut cards, &mut role_count);
    }
    */

    porker::million_porker(&use_cards, &mut role_count, num);

    println!("{:?}", role_count);

    porker::debug_judge_role(&role_count, 1_000_000);
    println!("{:?}", now.elapsed());
    //仮
}
*/
