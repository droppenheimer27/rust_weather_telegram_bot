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
use models::*;
use rusqlite::{Connection, Result};
use rusqlite::types::ToSql;

trait Data {
    fn new(id: i32, name: String, city: String) -> Box<Self>;
    fn insert(&self);
    fn update(&self);
    fn select(id: i32) -> Box<Self>;
    fn delete(id: i32);
}

impl Data for models::User {
    fn new(id: i32, name: String, city: String) -> Box<Self> {
        let user: models::User = models::User { id, name, city };

        return Box::new(user);
    }
    fn insert(&self) {
        let mut connect = Connection::open("test.db").unwrap();
        let transaction = connect.transaction().unwrap();

        transaction.execute("INSERT INTO users (user_id, user_name) VALUES (?1, ?2)",&[&self.id, &self.name as &ToSql]).unwrap();
        transaction.execute("INSERT INTO cities (cities_title, user_id) VALUES (?1, ?2)",&[&self.city as &ToSql, &self.id]).unwrap();

        transaction.commit();
    }

    fn update(&self) {
        unimplemented!()
    }

    fn select(id: i32) -> Box<Self> {
        let mut connect = Connection::open("test.db").unwrap();
        let mut statement = connect.prepare("SELECT cities_title FROM cities WHERE user_id = ?1").unwrap();
        let mut rows = statement.query(&[&id]).unwrap();

        let city: String = rows.next().unwrap().unwrap().get(0);

        let user = models::User {
            id: 0,
            name: String::from("aasd"),
            city: city
        };

        return Box::new(user);
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
    let mut dialog_status = DialogStatus::NotAsked;

    let handle = core.handle();
    let token = std::env::var(constants::TOKEN).unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                println!("<{}>: {}", &message.from.first_name, data);

                if data == constants::SAVE_ME_COMMAND {
                    api.spawn(message.text_reply(format!("Okay, input city now!")));
                    dialog_status = DialogStatus::SaveMeAsked;
                } else if data == constants::WEATHER_RECALL_COMMAND {
                    let user_id = message.from.id.to_string().parse::<i32>().unwrap();
                    let user: models::User = *Data::select(user_id);

                    let answer = get_temperature(user.city.as_str());
                    api.spawn(message.text_reply(answer));

                    dialog_status = DialogStatus::NotAsked;
                } else if data == constants::WEATHER_COMMAND {
                    api.spawn(message.text_reply(format!("Input city!")));

                    dialog_status = DialogStatus::WeatherAsked;
                } else {
                    match dialog_status {
                        DialogStatus::NotAsked => api.spawn(message.text_reply(format!("TEST!"))),
                        DialogStatus::SaveMeAsked => {
                            let user_id = message.from.id.to_string();
                            let user: models::User = *Data::new(user_id.parse::<i32>().unwrap(), message.clone().from.username.unwrap().to_string(), data.to_string());
                            user.insert();

                            dialog_status = DialogStatus::NotAsked;
                        },
                        DialogStatus::WeatherAsked => {
                            let answer = get_temperature(data);
                            api.spawn(message.text_reply(answer));

                            dialog_status = DialogStatus::NotAsked;
                        }
                    }
                }
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}