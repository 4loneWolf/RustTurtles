#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

//static GLOBAL: Lazy<Mutex<Vec<WebSocket<TcpStream>>>> = Lazy::new(|| {
//    let mut vec: Vec<WebSocket<TcpStream>> = Vec::new();
//    Mutex::new(vec)
//});

use std::net::TcpStream;
use rand::Rng;
use std::{net::TcpListener, thread, time::Duration, collections::HashMap};
use async_std::stream::StreamExt;
use serde::de::value;
use serde::ser::{Serialize, SerializeStruct};
use rocket::http::RawStr;
use std::{fs, array};
use std::io::prelude::*;
use serde_json::{Value, json};
use tungstenite::{accept, WebSocket, Message, accept_hdr};
use tungstenite::handshake::client;
use tungstenite::handshake::server::{Request, Response};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, Row, Pool, MySql};
use td_rlua::{Lua, LuaTable};
use sqlx;
use rocket::futures::TryStreamExt;
use tokio;
use tokio::sync::{Mutex, OnceCell};


// use rocket::request::Form;
use rocket::{http::{Cookie, CookieJar, Status}, fs::FileServer};
use rocket::serde::json::{Json};

static GLOBALL: Lazy<Mutex<HashMap<String, WebSocket<TcpStream>>>> = Lazy::new(|| {
    let mut hash: HashMap<String, WebSocket<TcpStream>> = HashMap::new();
    Mutex::new(hash)
});

static ELEVATOR: Lazy<Mutex<HashMap<String, WebSocket<TcpStream>>>> = Lazy::new(|| {
    let mut hash: HashMap<String, WebSocket<TcpStream>> = HashMap::new();
    Mutex::new(hash)
});

static used_turtles_mutex: Lazy<Mutex<HashMap<String, bool>>> = Lazy::new(|| {
    let mut hash: HashMap<String, bool> = HashMap::new();
    Mutex::new(hash)
});

//static DB: Lazy<Mutex<Pool<MySql>>> = Lazy::new(|| {
//    let pool = sqlx::MySqlPool::connect("mysql://orangepi:stepansysoev2006@192.168.0.109/turtles").await.unwrap();
//    Mutex::new(pool)
//});

static DB: OnceCell<Pool<MySql>> = OnceCell::const_new();
 
async fn init_sql_connection() -> Pool<MySql>{
    let pool = sqlx::MySqlPool::connect("mysql://orangepi:stepansysoev2006@192.168.0.109/turtles").await.unwrap();
    pool
}

// for base route 
#[get("/")]
fn api() -> String {
    format!("Hello, from Rust  \n Hit the back button to continue")
}

#[derive(Debug)] 
pub struct ApiResponse {
    pub body: Json<Value>,
    pub status: Status,
}

#[get("/message", rank = 1)]
fn json_message() ->  Value {
    json!({
        "app" : "svelte-on-rust",
        "version" : "0.0.1",
        "status": "Feeling Good"
    })
}

// check if user is logged in 
#[get("/checkuser")]
fn check_user(cookies: &CookieJar<'_>) -> Value {
    let user_id: String;
    match cookies.get_private("session_id") {
        Some(cookie) => user_id = cookie.value().to_string(),
        None => user_id = '_'.to_string(),
    }
    json!({"user_id": user_id})
}

// common client and server route
#[get("/common", rank = 1)]
fn common() ->  Value {
    json!({
        "app" : "svelte-on-rust",
        "version" : "0.0.1",
        "status": "Feeling Good"
    })
}

// User struct for login
#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}

#[post("/login", format = "json", data = "<user>")]
fn login_user(cookies: &CookieJar<'_>, user: Json<User>) -> Value {
    // should be replaced with databased logic and encrypted passwords
    if user.username == user.password {
        cookies.add_private(Cookie::new("session_id", user.into_inner().username));
        json!({ "result" : "success",
                "messge" : "login successfull"
        })
    }else {
        json!({ "result" : "error", 
                "message": "Invalid Username/Password"
        })
    }
}

#[get("/logout")]
fn logout_user(cookies: &CookieJar<'_>) -> Value {
    cookies.remove_private(Cookie::named("session_id"));
    json!({ "result" : "success",
            "messge" : "logout successfull"
    })
}

//secured area content 
#[get("/secure")]
fn secure_content(cookies: &CookieJar<'_>) -> Value {
    // fectch private date for the user based on session_id 
    match cookies.get_private("session_id"){
        Some(_cookie)=> json!({ "result": "success", "message": "this is your ultra secret private data"}),
        None => json!( {"result" : "error", "message": "no cookies for you"}),
    }
}




#[launch]
async fn rocket() -> _ {
    let used_turtles: HashMap<String, bool> = HashMap::new();
    let server = TcpListener::bind("192.168.0.108:34198").unwrap();

    tokio::spawn( async move {
        for stream in server.incoming() {
            //tokio::spawn( async move {
            let a = tokio::task::spawn(tokio::spawn(async {
                println!("started");
                //dbg!("{}", &stream);
                let mut websocket = accept(stream.unwrap()).unwrap(); //MAIN

                let message = websocket.read_message();

                if message.is_err() {
                    println!("{:?}", message.err());
                } else {
                    let msg = message.unwrap(); //MAIN

                    println!("{}", msg); //MAIN

                    let first_message_json: serde_json::Value = serde_json::from_str(msg.to_text().unwrap()).expect("Initial JSON request could not be parsed"); //MAIN
                    if first_message_json["TurtleName"] == "ElevatorControl" {
                        loop {
                            eprintln!("FWEFWEKFJWELFWELKFWELKFWEKLFWEKFJWEKLFL");
                            let message = websocket.read_message().unwrap();
                            let json: serde_json::Value = serde_json::from_str(message.to_text().unwrap()).expect("Elevator control (JSON fail to parse)");
                            let level: i64 = json["level"].as_i64().unwrap();
                            let toWhatFloor: i64 = json["ToWhatFloor"].as_i64().unwrap();
                            eprintln!("{}, {}", level, toWhatFloor);
                            if level > toWhatFloor {
                                let to = toWhatFloor.to_string() + "_bottom" ;
                                let mut binding = ELEVATOR.lock().await; //MAIN
                                let ws = binding.get_mut(&to).unwrap(); 
                                
                                let message = json!({
                                    "whatToPlace": "magma"
                                });

                                ws.write_message(Message::Text(message.to_string()));
                            } else if level < toWhatFloor {
                                let from = level.to_string() + &"_bottom".to_string();
                                let to = toWhatFloor.to_string() + &"_top".to_string();

                                let mut binding = ELEVATOR.lock().await; //MAIN
                                if toWhatFloor != -1 {
                                    let ws = binding.get_mut(&to).unwrap();
                                    let message = json!({
                                        "action": "place"
                                    });
                                    ws.write_message(Message::Text(message.to_string()));
                                }
                                let ws = binding.get_mut(&from).unwrap();
                                let message = json!({
                                    "whatToPlace": "soulsand"
                                });
                                ws.write_message(Message::Text(message.to_string()));
                            }
                        }
                    } else if first_message_json["TurtleName"] == "Elevator" {
                        let name = first_message_json["Level"].as_str().unwrap().to_string();
                        ELEVATOR.lock().await.insert(name, websocket);
                    } else {
                        let (name, need_to_send) = start_message_json(first_message_json).await; //MAIN
                        if need_to_send {
                            websocket.write_message(Message::Text(String::from(&name)));
                        }
                        println!("{}", name);
                        GLOBALL.lock().await.insert(name, websocket); //MAIN
                    }
                }
            }));
            //a.await;
        //});
        }
    });

    #[post("/", data="<message>")] //&<TurtleX>&<TurtleY>&<TurtleZ>&<TurtleDirection>
    async fn turtle_main(message: String) { //, TurtleX: i32, TurtleY: i32, TurtleZ: i32, TurtleDirection: String
        println!("{}", message);

        let json_message = parse_json(message.to_string());
        let turtle_name = json_message["name"].as_str().unwrap().to_string();
        let message = json_message["message"].as_str().unwrap().to_string();

        let mut binding = GLOBALL.lock().await; //MAIN
        let ws = binding.get_mut(&turtle_name).unwrap(); //MAIN

        main_func(ws, turtle_name, message).await;
    }

    #[post("/utility", data="<JSON>")]
    async fn init_for_site(JSON: String) -> Value {
        let processed_json = parse_json(JSON);

        let mut binding = GLOBALL.lock().await; //MAIN
        let ws = binding.get_mut(processed_json["message"]["name"].as_str().unwrap()).unwrap(); //MAIN

        let message = json!({
            "message": "inventory"
        });
        ws.write_message(Message::Text(String::from(message.to_string())));
        let res = ws.read_message();

        let response = res.unwrap();

        println!("{:?}", &response);

        let (x, y, z, direction) = getting_turtle_cords_for_init(&processed_json["message"]["name"].as_str().unwrap().to_string()).await;

        println!("{}, {}, {}, {}", x, y, z, direction );

        json!({
            "direction": direction, "x": x, "y": y, "z": z, "array": {}, "inventory": response.to_string()
        })
    }

    #[post("/getInventory", data="<JSON>")]
    async fn inventory(JSON: String) -> Value {
        let processed_json = parse_json(JSON);

        let mut binding = GLOBALL.lock().await; //MAIN
        let ws = binding.get_mut(processed_json["message"]["name"].as_str().unwrap()).unwrap(); //MAIN

        let message = json!({
            "message": "inventory"
        });
        ws.write_message(Message::Text(message.to_string())).expect("error sending a message to the turtle (inventory function)");
        let res = ws.read_message().expect("Error in inventory");

        let (x, y, z, direction) = getting_turtle_cords_for_init(&processed_json["message"]["name"].as_str().unwrap().to_string()).await;
        println!("{:?}", parse_json(res.to_string()));
        json!({
            "inventory": parse_json(res.to_string()), "x": x, "y": y, "z": z, "direction": direction
        }) // TEMP CORDS
    }

    #[post("/movement", data="<JSON>")]
    async fn movement(JSON: String) -> Value {
        let processed_json = parse_json(JSON);
        println!("{:?}", processed_json);
        let temp_message = processed_json["message"].as_str().unwrap();
        let message = parse_json(temp_message.to_string());
        let turtle_name = message["turtle_name"].as_str().unwrap();

        let mut binding = GLOBALL.lock().await; //MAIN
        let mut ws = binding.get_mut(turtle_name); //MAIN
        let mut boolean = false; let mut turtle_site_x = 0_i64; let mut turtle_site_y = 0_i64; let mut turtle_site_z = 0_i64; let mut direction = "none".to_string(); let mut turtle_x = 0_i64; let mut turtle_y = 0_i64; let mut turtle_z = 0_i64; let mut musor: String = "mysor".to_string();
        let mut top_block = "none"; let mut front_block = "none"; let mut bottom_block = "none"; let ws_message; let mut top_block_color = "none".to_string(); let mut front_block_color = "none".to_string(); let mut bottom_block_color = "none".to_string();
        let count = message["count"].as_str();
        let toSlot = message["toSlot"].as_str();
        if ws.is_some() {
            if check_ws_conn(ws.as_mut().unwrap()).await {
                let ws = ws.unwrap();
                let to_send_message;
                if count.is_some() {
                    to_send_message = json!({
                        "message": message["action"].as_str().unwrap(), "count": count.unwrap()
                    });
                } else if toSlot.is_some() {
                    to_send_message = json!({
                        "message": message["action"].as_str().unwrap(), "count": count.unwrap(), "toSlot": toSlot.unwrap()
                    });
                } else {
                    to_send_message = json!({
                        "message": message["action"].as_str().unwrap()
                    });
                }
                ws.write_message(Message::Text(to_send_message.to_string())).expect("error sending a message to the turtle in inventory function");
                let ws_req = ws.read_message();
                if !(ws_req.is_err()) {
                    let ws_string = ws_req.unwrap().to_string();
                    ws_message = parse_json(ws_string);
                    top_block = ws_message["up"].as_str().unwrap();
                    front_block = ws_message["middle"].as_str().unwrap(); bottom_block = ws_message["down"].as_str().unwrap();
                    let temp_boolean = ws_message["boolean"].as_bool().unwrap(); let action = ws_message["action"].as_str().unwrap();
                    if temp_boolean == true {
                        boolean = true
                    } else {
                        boolean = false
                    }

                    (top_block_color, front_block_color, bottom_block_color) = block_colors(&top_block.to_string(), &front_block.to_string(), &bottom_block.to_string()).await;
                    (turtle_x, turtle_y, turtle_z, direction) = getting_turtle_cords(&turtle_name.to_string()).await;
                    //println!("{:?}", message);
                    (turtle_site_x, turtle_site_y, turtle_site_z, musor) = getting_turtle_cords_for_init(&turtle_name.to_string()).await;

                    if boolean == true {
                        (turtle_x, turtle_y, turtle_z, turtle_site_x, turtle_site_y, turtle_site_z, direction) = handle_turtle_movement(turtle_x, turtle_y, turtle_z, turtle_site_x, turtle_site_y, turtle_site_z, &direction, action.to_string());
                        let pool = DB.get_or_init(init_sql_connection).await;
                        sqlx::query("UPDATE turtles_base SET turtle_x = ?, turtle_y = ?, turtle_z = ?, turtle_site_x = ?, turtle_site_y = ?, turtle_site_z = ?, direction = ? WHERE name = ?").bind(&turtle_x).bind(&turtle_y).bind(&turtle_z).bind(&turtle_site_x).bind(turtle_site_y).bind(turtle_site_z).bind(&direction).bind(&turtle_name).execute(pool).await.expect("could not delete blocks from the DB");
                    }

                    let pool = DB.get_or_init(init_sql_connection).await;
                    println!("{}", &front_block);
                    handle_inspected_blocks(turtle_x, turtle_y + 1, turtle_z, turtle_site_x, turtle_site_y + 1, turtle_site_z, top_block.to_string(), &pool, turtle_name.to_string()).await;

                    let (block_x, block_y, block_z, block_site_x, block_site_y, block_site_z, block_direction) = handle_turtle_movement(turtle_x, turtle_y, turtle_z, turtle_site_x, turtle_site_y, turtle_site_z, &direction, action.to_string());
                    handle_inspected_blocks(block_x, block_y, block_z, block_site_x, block_site_y, block_site_z, front_block.to_string(), &pool, turtle_name.to_string()).await;

                    handle_inspected_blocks(turtle_x, turtle_y - 1, turtle_z, turtle_site_x, turtle_site_y - 1, turtle_site_z, bottom_block.to_string(), &pool, turtle_name.to_string()).await;
                } else {
                    eprintln!("Could not unwrap turtles message in movement")
                }
            } else {
                println!("{} websocket is closed", turtle_name);
                binding.remove(turtle_name);
            }
        } else {
            eprintln!("No turtle with that name connected")
        }

        eprintln!("{}", &direction);

        json!({
            "colorUp": top_block_color, "colorMiddle": front_block_color, "colorDown": bottom_block_color, "x": turtle_site_x, "y": turtle_site_y, "z": turtle_site_z, "boolean": boolean , "up": top_block, "down": bottom_block, "middle": front_block, "direction": direction
        })
    }

    #[post("/UserConnected", data="<JSON>")]
    async fn handle_turtle_conn(JSON: String) {
        let processed_json = parse_json(JSON);

        let mut binding = GLOBALL.lock().await; //MAIN
        let turtle_name = processed_json["name"].as_str().unwrap();
        let mut ws = binding.get_mut(turtle_name); //MAIN

        if ws.is_some() {
            if check_ws_conn(ws.as_mut().unwrap()).await {
                let mut binding = used_turtles_mutex.lock().await; //MAIN
                binding.insert(turtle_name.to_string(), true);
            } else {
                binding.remove(turtle_name);
            }
        } else {
        }
    }

    #[post("/drop", data="<JSON>")]
    async fn drop(JSON: String) {
        
    }

    #[post("/getFloorsForElevators", data="<JSON>")]
    async fn getFloors(JSON: String) {
        
    }

    #[post("/select_and_transfer", data="<JSON>")]
    async fn select_slot(JSON: String) -> Value {
        let processed_json = parse_json(JSON);
        println!("{:?}", processed_json);
        let temp_message = processed_json["message"].as_str().unwrap();
        let message = parse_json(temp_message.to_string());
        let turtle_name = message["turtle_name"].as_str().unwrap();

        let mut binding = GLOBALL.lock().await; //MAIN
        let mut ws = binding.get_mut(turtle_name); //MAIN

        if ws.is_some() {
            if check_ws_conn(ws.as_mut().unwrap()).await {
                let ws = ws.unwrap();
                let to_send_message;
                if message["toSlot"].as_str().is_some() {
                    to_send_message = json!({
                        "message": message["action"].as_str().unwrap(), "toSlot": message["toSlot"].as_str().unwrap(), "count": message["count"].as_str().unwrap()
                    })
                } else {
                    to_send_message = json!({
                        "message": message["action"].as_str().unwrap()
                    });
                }

                ws.write_message(Message::Text(to_send_message.to_string())).expect("error sending a message to the turtle in inventory function");
                let ws_req = ws.read_message();
            }
        }
        

        json!({
            "boolean": true
        })

    }

    #[post("/UserDisconnected", data="<JSON>")]
    async fn handle_turtle_disconn(JSON: String) {
        let processed_json = parse_json(JSON);

        let mut binding = GLOBALL.lock().await; //MAIN
        let turtle_name = processed_json["name"].as_str().unwrap();
        let mut ws = binding.get_mut(turtle_name); //MAIN

        if ws.is_some() {
            if check_ws_conn(ws.as_mut().unwrap()).await {
                let mut binding = used_turtles_mutex.lock().await; //MAIN
                binding.insert(turtle_name.to_string(), false);
            } else {
                binding.remove(turtle_name);
            }
        } else {
        }
    }

    rocket::build()
    .mount("/api", routes![api,json_message,check_user,secure_content])
    .mount("/auth", routes![login_user,logout_user])
    .mount("/app", routes![common])
    .mount("/turtle", routes![turtle_main, init_for_site, inventory, movement, handle_turtle_disconn, handle_turtle_conn, select_slot])
    .mount("/", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
}


async fn main_func(ws: &mut WebSocket<TcpStream>, turtle_name: String, message: String) {
    let message = json!({
        "message": message
    });
    let res = ws.write_message(Message::Text(String::from(message.to_string())));
    if res.is_err() {
        eprintln!("{:?}", res.err());
    } else {
        let response = ws.read_message().unwrap();

        let msg = parse_json(response.to_string());

        println!("{}", msg.to_string());
        let (turtle_x, turtle_y, turtle_z, direction) = getting_turtle_cords(&turtle_name).await;
        println!("{}, {}, {}, {}", turtle_x, turtle_y, turtle_z, direction);
    };
}

async fn start_message_json(json_message: serde_json::Value) -> (String, bool) {
    let turtle_name: String; let mut turtle_x: i64; let mut turtle_y: i64; let mut turtle_z: i64; let mut turtle_site_x: i64 = 0; let mut turtle_site_y: i64 = 0; let mut turtle_site_z: i64 = 0; let mut direction: String;
    let pool = DB.get_or_init(init_sql_connection).await;
    let mut boolean = false;
    if !(json_message["TurtleName"] == "none") {
        turtle_name = json_message["TurtleName"].as_str().unwrap().to_string();
    } else {
        let mut result = sqlx::query("SELECT name FROM greek_names WHERE is_taken = FALSE").fetch_one( pool).await.unwrap();
        turtle_name = result.get("name");

        // ??????? WRITE MESSAGE BACK TO TURTLE WITH ITS NEW NAME ?????????

        sqlx::query("UPDATE greek_names SET is_taken = TRUE WHERE name = ?").bind(&turtle_name).execute(pool).await.expect("Could not set the names IS_TAKEN status to TRUE");
        boolean = true;
    }

    if !(json_message["TurtleX"] == "none") {
        turtle_x = json_message["TurtleX"].as_i64().unwrap();
        turtle_y = json_message["TurtleY"].as_i64().unwrap();
        turtle_z = json_message["TurtleZ"].as_i64().unwrap();
        direction = json_message["direction"].as_str().unwrap().to_string();
        sqlx::query("INSERT INTO turtles_base(name, turtle_x, turtle_y, turtle_z, turtle_site_x, turtle_site_y, turtle_site_z, direction) VALUES(?, ?, ?, ?, 0, 0, 0, ?)").bind(&turtle_name).bind(&turtle_x).bind(&turtle_y).bind(&turtle_z).bind(&direction).execute(pool).await.expect("");
    }

    return (turtle_name, boolean);
}

fn parse_json(json_txt: String) -> serde_json::Value {
    let first_message_json: serde_json::Value = serde_json::from_str(json_txt.as_str()).expect("JSON request could not be parsed (fn parse_json)");
    first_message_json
}

async fn getting_turtle_cords_for_init(name: &String) -> (i64, i64, i64, String) {
    let pool = DB.get_or_init(init_sql_connection).await;
    let result = sqlx::query("SELECT * FROM turtles_base WHERE name = ?").bind(name).fetch_one( pool).await.unwrap();

    let turtle_x: i64 = result.get("turtle_site_x");
    let turtle_y: i64 = result.get("turtle_site_y");
    let turtle_z: i64 = result.get("turtle_site_z");
    let direction: String = result.get("direction");

    println!("{}", direction);

    return (turtle_x, turtle_y, turtle_z, direction)
}

async fn getting_turtle_cords(name: &String) -> (i64, i64, i64, String) {
    let pool = DB.get_or_init(init_sql_connection).await;
    let result = sqlx::query("SELECT * FROM turtles_base WHERE name = ?").bind(name).fetch_one( pool).await.unwrap();

    let turtle_x: i64 = result.get("turtle_x");
    let turtle_y: i64 = result.get("turtle_y");
    let turtle_z: i64 = result.get("turtle_z");
    let direction: String = result.get("direction");

    println!("{}", direction);

    return (turtle_x, turtle_y, turtle_z, direction)
}

async fn check_ws_conn(ws: &mut &mut WebSocket<TcpStream>) -> bool {
    let message = json!({
        "message": "you good?"
    });
    ws.write_message(Message::Text(message.to_string())).is_err();

    if ws.write_message(Message::Text(message.to_string())).is_err() {
        println!("conn is closed");
        return false;
    } else {
        println!("conn is open");
        return true;
    }
}

async fn block_colors(top_block: &String, front_block: &String, bottom_block: &String) -> (String, String, String) {
    let pool = DB.get_or_init(init_sql_connection).await;
    let mut top_block_color = "none".to_string(); let mut front_block_color = "none".to_string(); let mut bottom_block_color = "none".to_string();

    if !(top_block == "none") {
        let result = sqlx::query("SELECT color FROM block_colors WHERE block_name = ?").bind(&top_block).fetch_one(pool).await;
        if result.is_err() {
            top_block_color = generate_hex_color();
            sqlx::query("INSERT INTO block_colors(block_name, color) VALUES(?, ?)").bind(&top_block).bind(&top_block_color).execute(pool).await.expect("Could not set the names IS_TAKEN status to TRUE");
        } else {
            top_block_color = result.unwrap().try_get("color").unwrap_or("none".to_string());
        }
    }
    if !(front_block == "none") {
        let result = sqlx::query("SELECT color FROM block_colors WHERE block_name = ?").bind(&front_block).fetch_one( pool).await;
        if result.is_err() {
            front_block_color = generate_hex_color();
            sqlx::query("INSERT INTO block_colors(block_name, color) VALUES(?, ?)").bind(&front_block).bind(&front_block_color).execute(pool).await.expect("Could not set the names IS_TAKEN status to TRUE");
        } else {
            front_block_color = result.unwrap().try_get("color").unwrap_or("none".to_string());
        }
    }   
    if !(bottom_block == "none") {
        let result = sqlx::query("SELECT color FROM block_colors WHERE block_name = ?").bind(&bottom_block).fetch_one( pool).await;
        if result.is_err() {
            bottom_block_color = generate_hex_color();
            sqlx::query("INSERT INTO block_colors(block_name, color) VALUES(?, ?)").bind(&bottom_block).bind(&bottom_block_color).execute(pool).await.expect("Could not set the names IS_TAKEN status to TRUE");
        } else {
            bottom_block_color = result.unwrap().try_get("color").unwrap_or("none".to_string());
        }
    }
    return (top_block_color, front_block_color, bottom_block_color)
}

    struct Block {
    block: String,
    block_x: i64,
    block_y: i64,
    block_z: i64,
    block_site_x: i64,
    block_site_y: i64,
    block_site_z: i64,
    block_color: String
}

async fn elevator_control() {

}

fn generate_hex_color() -> String {
    let chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let mut rng = rand::thread_rng();

    let mut top_block_color = "0x".to_owned();

    let mut color = "0x".to_string();
    for i in 1..7 {
        color = format!("{}{}", color.to_owned(), chars[rng.gen_range(0..16)]);
    }
    color
}

async fn getting_turtle_blocks(name: String) -> Vec<Block> {
    let mut all_blocks: Vec<Block> = Vec::new();
    let pool = DB.get_or_init(init_sql_connection).await;
    let result = sqlx::query("SELECT * FROM turtle_blocks WHERE name = ?").bind(&name).fetch_all( pool).await.unwrap();
    //if result.is_empty() // IMPORTANT?
    for row in result {
        let block_name = row.try_get("block").unwrap();
        let result2 = sqlx::query("SELECT color FROM block_colors WHERE block_name = ?").bind(&block_name).fetch_one( pool).await.expect("Could not get blocks info for a turtle");
        let block_info = Block {
            block: block_name,
            block_x: row.try_get("block_x").unwrap(),
            block_y: row.try_get("block_y").unwrap(),
            block_z: row.try_get("block_z").unwrap(),
            block_site_x: row.try_get("block_site_x").unwrap(),
            block_site_y: row.try_get("block_site_y").unwrap(),
            block_site_z: row.try_get("block_site_z").unwrap(),
            block_color: result2.get("color")
        };
        all_blocks.push(block_info);
    }
    all_blocks
}

fn forward(x: i64, z: i64, direction: String) -> (i64, i64) {
    let mut newXx = x; let mut newZz = z;
    if direction == "north" {
        newZz = z - 1;
        if z == 1 && newZz == 0 {
            newZz = -1;
        }
    } else if direction == "south" {
        newZz = z + 1;
        if newZz == 0 && z == -1 {
            newZz = 1;
        }
    } else if direction == "west" {
        newXx = x - 1;
        if x == 1 && newXx == 0 {
            newXx = -1;
        }
    } else if direction == "east" {
        newXx = x + 1;
        if x == -1 && newXx == 0 {
            newXx = 1;
        }
    } else {
        eprintln!("Not a direction (at forward function)");
    }
    return (newXx, newZz);
}

fn forward_site(mut newX: i64, mut newZ: i64, direction: String) -> (i64, i64) {
    if direction == "north" {
        newZ = newZ + 1;
    }
    else if direction == "south" {
        newZ = newZ - 1;
    }
    else if direction == "west" {
        newX = newX + 1;
    }
    else if direction == "east" {
        newX = newX - 1;
    }
    else {
        eprintln!("Not a direction (at forward_site function)");
    }
    return (newX, newZ);
}

fn back(x: i64, z: i64, direction: String) -> (i64, i64) {
    let mut newXx = x; let mut newZz = z;
    if direction == "north" {
        newZz = z + 1;
        if z == -1 && newZz == 0 {
            newZz = 1;
        }
    } else if direction == "south" {
        newZz = z - 1;
        if newZz == 0 && z == 1 {
            newZz = -1;
        }
    } else if direction == "west" {
        newXx = x + 1;
        if x == -1 && newXx == 0 {
            newXx = 1;
        }
    } else if direction == "east" {
        newXx = x - 1;
        if x == 1 && newXx == 0 {
            newXx = -1;
        }
    } else {
        eprintln!("Not a direction (at back function)");
    }
    return (newXx, newZz);
}

fn back_site(mut newX: i64, mut newZ: i64, direction: String) -> (i64, i64) {
    if direction == "north" {
        newZ = newZ - 1
    } 
    else if direction == "south" {
        newZ = newZ + 1
    } 
    else if direction == "west" {
        newX = newX - 1
    } 
    else if direction == "east" {
        newX = newX + 1
    } 
    else {
        eprintln!("Not a direction (at forward_site function)");
    }
    return (newX, newZ);

}

fn left(direction: &String) -> String {
    let newDirection: String;
    println!("{}", &direction);
    if direction == &"north" {
        newDirection = "west".to_string();
    }
    else if direction == &"west".to_string() {
        newDirection = "south".to_string();
    }
    else if direction == &"south".to_string() {
        newDirection = "east".to_string();
    }
    else if direction == &"east".to_string() {
        newDirection = "north".to_string();
    }
    else {
        eprintln!("Not a direction (at left function)");
        newDirection = "none".to_string();
    }
    println!("{}", &newDirection);
    return newDirection;
}

fn right(direction: &String) -> String {
    let newDirection: String;
    if direction == &"north".to_string() {
        newDirection = "east".to_string();
    }
    else if direction == &"east".to_string() {
        newDirection = "south".to_string();
    }
    else if direction == &"south".to_string() {
        newDirection = "west".to_string();
    }
    else if direction == &"west".to_string() {
        newDirection = "north".to_string();
    }
    else {
        eprintln!("Not a direction (at right function)");
        newDirection = "none".to_string();
    }
    return newDirection;
}

fn handle_turtle_movement(mut x: i64, mut y: i64, mut z: i64, mut sitex: i64, mut sitey: i64, mut sitez: i64, mut direction: &String, action: String) -> (i64, i64, i64, i64, i64, i64, String) {
    let mut new_direction = direction.to_string();
    if action == "forward" {
        (x, z) = forward(x, z, direction.clone());
        (sitex, sitez) = forward_site(sitex, sitez, direction.clone())
    } else if action == "back" {
        (x, z) = back(x, z, direction.clone());
        (sitex, sitez) = back_site(sitex, sitez, direction.clone())
    } else if action == "left" {
        new_direction = left(&direction);
    } else if action == "right" {
        new_direction = right(&direction);
    } else if action == "up" {
        y = y + 1;
        sitey = sitey + 1;
    } else if action == "down" {
        y = y - 1;
        sitey = sitey - 1;
    }
    return (x, y, z, sitex, sitey, sitez, new_direction)
}

async fn handle_inspected_blocks(x: i64, y: i64, z: i64, site_x: i64, site_y: i64, site_z: i64, block_name: String, pool: &Pool<MySql>, turtle_name: String) {
    if block_name == "none" {
        sqlx::query("DELETE FROM turtle_blocks WHERE block_x = ? AND block_y = ? AND block_z = ?").bind(&x).bind(&y).bind(&z).execute(pool).await.expect("could not delete blocks from the DB");
    } else {
        let result = sqlx::query("SELECT * FROM turtle_blocks WHERE block_x = ? AND block_y = ? AND block_z = ?").bind(&x).bind(&y).bind(&z).fetch_one(pool).await;
        if result.is_err() {
            sqlx::query("INSERT INTO turtle_blocks(turtle_name, block, block_x, block_y, block_z, block_site_x, block_site_y, block_site_z) VALUES(?, ?, ?, ?, ?, ?, ?, ?)").bind(&turtle_name).bind(&block_name).bind(&x).bind(&y).bind(&z).bind(&site_x).bind(&site_y + 1).bind(&site_z).execute(pool).await.expect("could not update blocks name (top block func)");
        } else {
            sqlx::query("UPDATE turtle_blocks SET block = ? WHERE block_x = ? AND block_y = ? AND block_z = ?").bind(&block_name).bind(&x).bind(&y).bind(&z).execute(pool).await.expect("could not update blocks name");
        }
    }
}