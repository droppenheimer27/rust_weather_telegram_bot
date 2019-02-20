extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
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

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let token = std::env::var(constants::TOKEN).unwrap();

    let api = Api::configure(token).build(core.handle()).unwrap();

    let mut status = models::DialogStatus::WeatherNotAsked;
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
                    api.spawn(message.text_reply(format!("TEST!"))); // TODO: FINISH THIS
                } else if data == constants::WEATHER_COMMAND {
                    api.spawn(message.text_reply(format!("Input city!")));
                    status = models::DialogStatus::WeatherAsked;
                } else {
                    match status {
                        models::DialogStatus::WeatherNotAsked => api.spawn(message.text_reply(format!("TEST!"))),
                        models::DialogStatus::WeatherAsked => {
                            api.spawn(message.text_reply(get_location(data)));

                            status = models::DialogStatus::WeatherNotAsked;
                        }
                    }
                }
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}

fn get_location(city: &str) -> String {
    let url = format!("{url}?appid={key}&q={city}", url = constants::URL_API, key = constants::WEATHER_API_KEY, city = city);
    let mut response = reqwest::get(&url)
        .expect("Oh, shit!");
    let mut json = String::new();
    response.read_to_string(&mut json).expect("Failed to read response");
    let deserialized_data: models::ApiResponse = serde_json::from_str(&json).unwrap();
    let temp = utils::to_celsius(deserialized_data.main.temp);

    return String::from(format!("In {city} now is {temp}°C", city = city, temp = temp));
}