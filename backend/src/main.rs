#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use std::sync::Mutex;
use rocket::{State, http::Method};
use rocket_contrib::json::Json;
use rocket_cors::{CorsOptions, AllowedOrigins};

#[derive(Debug)]
struct Calculator {
    value: f32,
    stored_value: f32,
    operation: Option<Operation>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            stored_value: 0.0,
            operation: None,
        }
    }

    pub fn clear(&mut self) {
        self.value = 0.0;
        self.stored_value = 0.0;
        self.operation = None;
    }

    pub fn add_digit(&mut self, digit: u8) {
        self.value = self.value * 10.0 + f32::from(digit);
    }

    pub fn set_operation(&mut self, operation: Operation) {
        self.stored_value = self.value;
        self.value = 0.0;
        self.operation = Some(operation);
    }

    pub fn calculate(&mut self) -> Option<f32> {
        match self.operation {
            Some(Operation::Add) => {
                self.value = self.stored_value + self.value;
                self.operation = None;
                Some(self.value)
            }
            Some(Operation::Subtract) => {
                self.value = self.stored_value - self.value;
                self.operation = None;
                Some(self.value)
            }
            Some(Operation::Multiply) => {
                self.value = self.stored_value * self.value;
                self.operation = None;
                Some(self.value)
            }
            Some(Operation::Divide) => {
                if self.value != 0.0 {
                    self.value = self.stored_value / self.value;
                    self.operation = None;
                    Some(self.value)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[post("/clear")]
fn clear(calculator: State<Mutex<Calculator>>) {
    calculator.lock().unwrap().clear();
}

#[post("/add_digit/<digit>")]
fn add_digit(digit: u8, calculator: State<Mutex<Calculator>>) {
    calculator.lock().unwrap().add_digit(digit);
}

#[post("/operation/<operation>")]
fn operation(operation: String, calculator: State<Mutex<Calculator>>) -> String {
    match operation.as_str() {
        "+" => calculator.lock().unwrap().set_operation(Operation::Add),
        "-" => calculator.lock().unwrap().set_operation(Operation::Subtract),
        "*" => calculator.lock().unwrap().set_operation(Operation::Multiply),
        "/" => calculator.lock().unwrap().set_operation(Operation::Divide),
        _ => return "Invalid operation".to_string(),
    }
    "Operation set".to_string()
}

#[get("/calculate")]
fn calculate(calculator: State<Mutex<Calculator>>) -> Option<Json<f32>> {
    calculator.lock().unwrap().calculate().map(Json)
}

fn main() {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    rocket::ignite()
    .attach(cors.to_cors().unwrap())
        .mount("/", routes![clear, add_digit, operation, calculate])
        .manage(Mutex::new(Calculator::new()))
        .launch();
}
