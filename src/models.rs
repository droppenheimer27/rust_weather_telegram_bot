use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
   pub coord: Coord,
   pub weather: Vec<Weather>,
   pub base: String,
   pub main: Main,
   pub visibility: f32,
   pub wind: Wind,
   pub clouds: Clouds,
   pub dt: f32,
   pub sys: Sys,
   pub id: f32,
   pub name: String,
   pub cod: f32
}

#[derive(Serialize, Deserialize)]
pub struct Coord {
   pub lon: f32,
   pub lat: f32
}

#[derive(Serialize, Deserialize)]
pub struct Weather {
   pub id: i32,
   pub main: String,
   pub description: String,
   pub icon: String
}

#[derive(Serialize, Deserialize)]
pub struct Main {
   pub temp: f32,
   pub pressure: f32,
   pub humidity: f32,
   pub temp_min: f32,
   pub temp_max: f32
}

#[derive(Serialize, Deserialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32
}

#[derive(Serialize, Deserialize)]
pub struct Clouds {
    pub all: f32
}

#[derive(Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename="type")]
   pub _type: f32,
   pub id: f32,
   pub message: f32,
   pub country: String,
   pub sunrise: i64,
   pub sunset: i64
}

pub enum DialogStatus {
   WeatherNotAsked,
   WeatherAsked,
//   WeatherResponsed
}
