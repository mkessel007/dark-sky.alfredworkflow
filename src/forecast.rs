use std::fmt;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub enum Icon {
    ClearDay,
    ClearNight,
    Rain,
    Snow,
    Sleet,
    Wind,
    Fog,
    Cloudy,
    PartlyCloudyDay,
    PartlyCloudyNight,
    Unknown(String),
}

impl<'de> Deserialize<'de> for Icon {
    fn deserialize<D>(deserializer: D) -> Result<Icon, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "clear-day" => Icon::ClearDay,
            "clear-night" => Icon::ClearNight,
            "rain" => Icon::Rain,
            "snow" => Icon::Snow,
            "sleet" => Icon::Sleet,
            "wind" => Icon::Wind,
            "fog" => Icon::Fog,
            "cloudy" => Icon::Cloudy,
            "partly-cloudy-day" => Icon::PartlyCloudyDay,
            "partly-cloudy-night" => Icon::PartlyCloudyNight,
            s => Icon::Unknown(s.into()),
        })
    }
}


#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub currently: Option<Point>,
    pub minutely: Option<Block>,
}

#[derive(Debug, Deserialize)]
pub struct Point {
    #[serde(rename = "temperature")] pub temp: Option<f64>,
    #[serde(rename = "apparentTemperature")] pub apparent_temp: Option<f64>,
    pub icon: Option<Icon>,
    #[serde(rename = "precipIntensity")] precip_intensity: Option<f64>,
    #[serde(rename = "precipProbability")] precip_probability: Option<f64>,
    pub summary: Option<String>,
}

impl Point {
    pub fn precipitation(&self) -> Option<Precipitation> {
        self.precip_intensity.and_then(|intensity| {
            self.precip_probability.map(|probability| {
                Precipitation {
                    intensity,
                    probability,
                }
            })
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Block {
    data: Vec<Point>,
    pub summary: Option<String>,
    pub icon: Option<Icon>,
}

impl Block {
    // This isn't quite right since I'm just dropping precipitation values that aren't there...
    pub fn precipitations(&self) -> Vec<Precipitation> {
        self.data.iter().flat_map(|x| x.precipitation()).collect()
    }
}

pub struct Precipitation {
    pub intensity: f64,
    pub probability: f64,
}

impl Precipitation {
    fn human_intensity(&self) -> String {
        let intensity = if (0.0..=0.002).contains(self.intensity) {
            "no"
        } else if (0.002..=0.017).contains(self.intensity) {
            "very light"
        } else if (0.017..=0.1).contains(self.intensity) {
            "light"
        } else if (0.1..=0.4).contains(self.intensity) {
            "moderate"
        } else {
            "heavy"
        };
        intensity.into()
    }
}

impl fmt::Display for Precipitation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}% chance of {} rain.",
            (self.probability * 100.0).round(),
            self.human_intensity()
        )
    }
}
