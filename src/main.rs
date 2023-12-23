// API usage:
// Set value: curl -X POST http://localhost:8080/api/measurement -H "Content-Type: application/json" -d '{"sensor":"clock","value":112,"timestamp":"12:12"}'
// Querry value: curl http://localhost:8080/api/measurement

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;


struct AppState {
    //buffer: Vec<PulsMeasurement>,
    buffer_mutex: Mutex<Vec<Measurement>>,
}

#[derive(Serialize, Deserialize)]
struct Measurement {
    sensor: String,
    value: u16,
    timestamp: String,
}


#[get("/api/measurement")]
async fn get_measurement(state: web::Data<AppState>) -> impl Responder {
    let all_measurements = state.buffer_mutex.lock().unwrap();
    let last_measurement = all_measurements.last().unwrap();
    println!("Recieved measurement request");
    HttpResponse::Ok().body(serde_json::to_string(last_measurement.clone()).unwrap())
}

#[post("/api/measurement")]
async fn update_measurement(state: web::Data<AppState>, body: actix_web::web::Json<Measurement>) -> impl Responder {
    // state.buffer_mutex.lock().unwrap().push(Measurement { sensor: "puls".to_string(), value: 130, timestamp: "12:12".to_string() });
    state.buffer_mutex.lock().unwrap().push(Measurement { sensor: body.sensor.clone(), value: body.value, timestamp: body.timestamp.clone() });
    println!("Recieved update measurement request");
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut buffer = web::Data::new(AppState {
        buffer_mutex: Mutex::new(Vec::new()),
    });

    println!("Starting HTTP Server ...");
    HttpServer::new(move || {
        App::new()
            .app_data(buffer.clone())
            .service(update_measurement)
            .service(get_measurement)
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
