extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate rusqlite;
extern crate reqwest;
extern crate serde_derive;
extern crate serde_json;

mod constants;
mod models;
mod utils;

use futures::{Stream};
use tokio_core::reactor::{Core, Handle};
use telegram_bot::*;
use std::io::Read;
use models::{ApiResponse, Temperature, DialogStatus};
use rusqlite::{Connection, Result};
use rusqlite::types::ToSql;

trait Data {
    fn new(id: i32, name: String) -> Box<Self>;
    fn insert(&self);
    fn update(&self);
    fn select(id: i32) -> Box<Self>;
    fn delete(id: i32);
}

impl Data for models::User {
    fn new(id: i32, name: String) -> Box<Self> {
        let user: models::User = models::User { id, name };

        return Box::new(user);
    }
    fn insert(&self) {
        let connect = Connection::open("test.db").unwrap();

        connect.execute(
            "INSERT INTO users (user_id, user_name) VALUES (?1, ?2)",
            &[&self.id, &self.name as &ToSql]
        ).unwrap();
    }

    fn update(&self) {
        unimplemented!()
    }

    fn select(id: i32) -> Box<Self> {
        unimplemented!()
    }

    fn delete(id: i32) {
        unimplemented!()
    }
}

fn get_temperature(city: &str) -> String {
    let api = std::env::var(&constants::WEATHER_API_KEY).unwrap(); // TODO: CHECK THIS
    let url = format!("{url}?appid={key}&q={city}", url = constants::URL_API, key = api, city = city);

    let mut response = reqwest::get(&url).unwrap();
    let mut json = String::new();
    response.read_to_string(&mut json).unwrap();

    let deserialized_data: ApiResponse = serde_json::from_str(&json).unwrap();

    let temp = Temperature {
        city: deserialized_data.name,
        value: utils::to_celsius(deserialized_data.main.temp)
    };

//    insert().unwrap();
    return String::from(format!("In {city} now is {temp}°C", city = temp.city, temp = temp.value));
}

fn main() {
    let mut core = Core::new().unwrap();
    let mut status = models::DialogStatus::WeatherNotAsked;

    let handle = core.handle();
    let token = std::env::var(constants::TOKEN).unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                println!("<{}>: {}", &message.from.first_name, data);

//                match data.c_string() {
//                    constants::START_COMMAND => api.spawn(message.text_reply(format!("TEST!"))),
//                    constants::FIND_ME_COMMAND => get_location(api.clone(), message.clone(), handle.clone()),
//                    _ => api.spawn(message.text_reply(format!("TEST!")))
//                }
                if data == constants::START_COMMAND {
//                    let user = models::User {
//                        id: message.from.id as i32,
//                        name: message.from.username.unwrap()
//                    };
                    /* TODO: REMOVE SHITCODE, ULTRAREFACTOR AND NEED USER CHECKER */
                    let user_id = message.from.id.to_string();
                    let user: models::User = *Data::new(user_id.parse::<i32>().unwrap(), message.clone().from.username.unwrap().to_string());
                    user.insert();

                    api.spawn(message.text_reply(format!("Saved!"))); // TODO: FINISH THIS
                } else if data == constants::WEATHER_COMMAND {
                    println!("user: {}", message.from.id);
                    api.spawn(message.text_reply(format!("Input city!")));
                    status = models::DialogStatus::WeatherAsked;
                } else {
                    match status {
                        DialogStatus::WeatherNotAsked => api.spawn(message.text_reply(format!("TEST!"))),
                        DialogStatus::WeatherAsked => {
                            let answer = get_temperature(data);
                            api.spawn(message.text_reply(answer));

                            status = DialogStatus::WeatherNotAsked;
                        }
                    }
                }
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}