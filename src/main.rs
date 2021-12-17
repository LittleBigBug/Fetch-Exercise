// Simple program following guidelines on the Coding Exercise PDF

use crate::api_response::ApiResponse;

use chrono::prelude::*;

use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::response::content;
use rocket::http::Status;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rocket::serde::json::serde_json::json;
use crate::api_response::JsonObject::{ArbitraryJson, GuardJson};

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;


// Custom Rocket Response for handling JSON & errors

pub(crate) mod api_response;

//# Storage (in this case not a database/ORM)

/*
 / ARC and RwLock since Rocket is asynchronous
 / https://doc.rust-lang.org/std/sync/struct.RwLock.html
 / https://doc.rust-lang.org/std/sync/struct.Arc.html
*/

lazy_static! {
    static ref LAST_TRANSACTION: Arc<Mutex<Option<DateTime<Utc>>>> = Arc::new(Mutex::new(None));
    static ref TRANSACTIONS: Arc<Mutex<Vec<PointTransaction>>> = Arc::new(Mutex::new(Vec::new()));

    static ref CURRENT_CACHE: Arc<Mutex<Option<PointCache>>> = Arc::new(Mutex::new(None));
}

//# Data structures (JSON + storage)

#[derive(Debug, Clone)]
struct PointTransaction {
    payer: String,
    points: i32,
    timestamp: i64,
}

#[derive(Clone)]
struct PointCache {
    total_points: i32,
    sorted_transactions: Vec<PointTransaction>,
    points_by_payer: HashMap<String, i32>,
    timestamp: i64,
}

// JSON Guard structs for Rocket.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PointTransactionJSON<'a> {
    payer: String,
    points: i32,
    timestamp: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PointsSpendJSON {
    points: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PointsSpendRespJSON {
    payer: String,
    points: i32,
}

// Result type

type SpendResult = Result<HashMap<String, i32>, &'static str>;

//# Utility Functions

fn get_sorted_transactions() -> Vec<PointTransaction> {
    let transactions_read_lock = TRANSACTIONS.clone();
    let transactions = transactions_read_lock.lock().unwrap();

    // Ascending sort
    let mut transactions_sorted: Vec<PointTransaction> = transactions.to_vec();
    transactions_sorted.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    transactions_sorted
}

fn calculate_spend_points(spend_target: i32) -> SpendResult {
    let current_cache_read_lock = CURRENT_CACHE.clone();
    let current_cache = current_cache_read_lock.lock().unwrap();
    let active_cache = current_cache.clone().unwrap();

    if spend_target > active_cache.total_points {
        return Err("Not enough points!");
    }

    let transactions_sorted = active_cache.sorted_transactions;

    let mut spend_target_left = spend_target;
    let mut payer_points_spent: HashMap<String, i32> = HashMap::new();

    // This system takes into account "Scenario A" in Readme.MD
    let mut used_transactions: HashMap<u16, i32> = HashMap::new();

    // loop again if the spend target isn't satisfied
    while spend_target_left > 0
        && used_transactions.len() != transactions_sorted.len() {

        let mut ind: u16 = 0;

        // Loop thru every transaction in order of their timestamp
        for transaction in transactions_sorted.clone() {
            let payer = transaction.payer.clone();
            let points = transaction.points.clone();

            let mut max_use_points = points;

            // Ignore indices that we have already fully used
            // Only positive points can be partially used
            let used_before = used_transactions.contains_key(&ind);
            let used_amt_opt = used_transactions.get(&ind);

            if used_before {
                let used_amt = used_amt_opt.unwrap();
                let diff = points - used_amt;

                // Used completely
                if diff == 0 {
                    ind += 1;
                    continue;
                }

                max_use_points = diff;
            }

            let mut used = max_use_points;

            // Truncate points used if not all of them are needed
            if max_use_points >= spend_target_left {
                used = spend_target_left;
            }

            let mut new_used = used;
            let negative_and_exists;

            // https://github.com/rust-lang/rust/issues/59159
            {
                let existing_payer_spent = payer_points_spent.get(&payer);
                let previously_used =
                    if existing_payer_spent.is_some() { existing_payer_spent.unwrap() } else { &0 };

                new_used += previously_used;
                negative_and_exists = existing_payer_spent.is_some() && points < 0;
            }

            // Even if there is nothing left in our spending target, if there is negative points value from a
            // payer that we have gotten points towards this target for, we need to account for it
            if spend_target_left > 0 || negative_and_exists {
                spend_target_left -= used;
                payer_points_spent.insert(payer, new_used);

                used_transactions.insert(ind, used);

                // If negative, Break through this loop and start at the beginning to get the oldest
                // paying points.
                if negative_and_exists {
                    break;
                }
            }

            ind += 1;
        };
    }

    Ok(payer_points_spent)
}

fn calculate_transactions() {
    let transactions_sorted = get_sorted_transactions();

    let mut total_points: i32 = 0;
    let mut payer_points_total: HashMap<String, i32> = HashMap::new();

    for transaction in transactions_sorted.clone() {
        let payer = transaction.payer.clone();
        let points = transaction.points.clone();

        total_points += points;

        let mut new_points = points;

        if payer_points_total.contains_key(payer.as_str()) {
            let cur_points = payer_points_total.get(payer.as_str()).unwrap();
            new_points = cur_points + points;
        }

        payer_points_total.insert(payer, new_points);
    };

    let new_cache = PointCache {
        total_points,
        sorted_transactions: transactions_sorted,
        points_by_payer: payer_points_total,
        timestamp: Utc::now().timestamp_millis(),
    };

    let current_cache_write_lock = CURRENT_CACHE.clone();
    let mut current_cache = current_cache_write_lock.lock().unwrap();
    *current_cache = Some(new_cache);
}

fn cache_is_stale() -> Bool {
    let current_cache_read_lock = CURRENT_CACHE.clone();
    let current_cache = current_cache_read_lock.lock().unwrap();

    if current_cache.is_none() { true }

    let last_transaction_read_lock = LAST_TRANSACTION.clone();
    let last_transaction = last_transaction_read_lock.lock().unwrap();

    // No transactions in this case means there's nothing to cache or calculate
    if last_transaction.is_none() { false }

    let active_cache = current_cache.clone().unwrap();
    let current_cache_time = active_cache.timestamp;

    let last_transaction_time = last_transaction.unwrap().timestamp_millis();

    // If the cache is newer than or the same as the last transaction, not stale
    current_cache_time < last_transaction_time
}

fn insert_transactions(transactions: Vec<PointTransaction>) {
    let transaction_write_guard = TRANSACTIONS.clone();
    let mut transaction = transaction_write_guard.lock().unwrap();

    let last_transaction_write_guard = LAST_TRANSACTION.clone();
    let mut last_transaction = last_transaction_write_guard.lock().unwrap();

    for trs in transactions {
        transaction.push(trs);
    };

    *last_transaction = Some(Utc::now());
}

fn insert_transaction(transaction: PointTransaction) {
    let mut transactions_vec = Vec::new();
    transactions_vec.push(transaction);

    insert_transactions(transactions_vec);
}

//# Routes

#[post("/point-transaction", format = "application/json", data = "<transaction>")]
fn point_transaction(transaction: Json<PointTransactionJSON>) {
    // Assuming API keys are handled somewhere safely...
    let transaction_from_json = transaction.into_inner();

    // https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.parse_from_rfc3339
    // Parses ISO8601
    let timestamp_datetime =
        match DateTime::parse_from_rfc3339(transaction_from_json.timestamp) {
            Ok(date) => { date },
            // I could also just default the timestamp to now if its invalid
            Err(_) => { panic!("Couldn't convert timestamp") },
        };

    let transaction = PointTransaction {
        payer: transaction_from_json.payer,
        points: transaction_from_json.points,
        timestamp: timestamp_datetime.timestamp_millis(),
    };

    insert_transaction(transaction);
    calculate_transactions();
}

#[get("/points", format = "application/json")]
fn get_points() -> Json<HashMap<String, i32>> {
    if cache_is_stale() {
        calculate_transactions();
    }

    let current_cache_read_lock = CURRENT_CACHE.clone();
    let current_cache = current_cache_read_lock.lock().unwrap();
    let points_cache = current_cache.clone().unwrap();

    // Copy Hashmap
    let mut points_by_payer: HashMap<String, i32> = HashMap::new();
    points_by_payer.clone_from(&points_cache.points_by_payer);

    Json(points_by_payer)
}

#[get("/points-sum", format = "application/json")]
fn get_points_total() -> content::Json<String> {
    if cache_is_stale() {
        calculate_transactions();
    }

    let current_cache_read_lock = CURRENT_CACHE.clone();
    let current_cache = current_cache_read_lock.lock().unwrap();
    let points_cache = current_cache.clone().unwrap();

    let total_points = points_cache.total_points;

    content::Json(format!("{{ \"total-points\": {} }}", total_points))
}

#[post("/spend-points", format = "application/json", data = "<points_spend>")]
fn spend_points_json(points_spend: Json<PointsSpendJSON>) -> ApiResponse<Vec<PointsSpendRespJSON>> {
    spend_points(points_spend.points)
}

#[post("/spend-points/<points>", format = "application/json")]
fn spend_points(points: i32) -> ApiResponse<Vec<PointsSpendRespJSON>> {
    // Can't spend negative or 0 points
    if points <= 0 {
        return ApiResponse {
            json: GuardJson(Json(Vec::new())),
            status: Status::Ok,
        };
    }

    if cache_is_stale() {
        calculate_transactions();
    }

    match calculate_spend_points(points) {
        Ok(spent_points) => {
            let mut transactions: Vec<PointTransaction> = Vec::new();

            // The test asked I display it this way
            let mut spent_points_negative: Vec<PointsSpendRespJSON> = Vec::new();

            for (payer, points) in spent_points.into_iter() {
                transactions.push(PointTransaction {
                    payer: payer.clone().into(),
                    points: points.abs() * -1,
                    timestamp: Utc::now().timestamp_millis(),
                });

                spent_points_negative.push(PointsSpendRespJSON {
                    payer,
                    points: points.abs() * -1
                });
            };

            insert_transactions(transactions);

            ApiResponse {
                json: GuardJson(Json(spent_points_negative)),
                status: Status::Ok,
            }
        },
        Err(e) => {
            ApiResponse {
                json: ArbitraryJson(json!({ "status": "error", "message": e })),
                status: Status::BadRequest,
            }
        },
    }
}

#[get("/")]
fn index() -> &'static str {
    "\
    Hello! Please check the Readme.md for info
    "
}

//# Rocket launch (Main application loop)
// https://rocket.rs

#[rocket::main]
async fn main() {
    match rocket::build()
        .mount("/", routes![index, spend_points, spend_points_json, get_points, get_points_total, point_transaction])
        .launch()
        .await
    {
        Ok(_) => { },
        Err(e) => {
            panic!("There was an error trying to launch Rocket! : {}", e.to_string());
        }
    }
}