use warp::Filter;
use serde::{Deserialize, Serialize};
use fhe_core::*;

#[derive(Deserialize)]
pub struct SumRequest {
    pub values: Vec<u64>,
}

#[derive(Serialize)]
pub struct SumResponse {
    pub sum: u64,
    pub status: String,
}

pub async fn run_server() {
    // /api/sum Endpoint
    let sum_route = warp::path!("api" / "sum")
        .and(warp::post())
        .and(warp::body::json())
        .map(|req: SumRequest| {
            // Hier später FHE-Operationen einfügen
            let sum: u64 = req.values.iter().sum();
            warp::reply::json(&SumResponse {
                sum,
                status: "success".to_string(),
            })
        });

    // /health Endpoint
    let health_route = warp::path!("health")
        .map(|| "FHE API is running");

    let routes = sum_route.or(health_route);

    println!("🚀 FHE API Server starting on http://localhost:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
