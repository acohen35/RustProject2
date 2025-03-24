use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "d0fa3961aa4b54de6c397d7052c323a4"; // Replace with your API key
    let client = reqwest::Client::new();

    loop {
        println!("\nWeather App - Select an option:");
        println!("1. Current Weather");
        println!("2. 5-Day Forecast");
        println!("3. Air Quality");
        println!("4. Weather Alerts");
        println!("5. Compare Multiple Locations");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        match choice.trim() {
            "1" => get_current_weather(&client, api_key).await?,
            "2" => get_forecast(&client, api_key).await?,
            "3" => get_air_quality(&client, api_key).await?,
            "4" => get_weather_alerts(&client, api_key).await?,
            "5" => compare_locations(&client, api_key).await?,
            "6" => break,
            _ => println!("Invalid option, please try again"),
        }
    }
    Ok(())
}

// Helper function to validate US ZIP codes
fn is_valid_zip(zip: &str) -> bool {
    zip.len() == 5 && zip.chars().all(|c| c.is_ascii_digit())
}

// Data structures for API responses
#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    wind: Wind,
    name: String,  // Added to get location name from ZIP
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    humidity: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
}

#[derive(Debug, Deserialize)]
struct ForecastResponse {
    list: Vec<ForecastItem>,
    city: City,  // Added to get location name from ZIP
}

#[derive(Debug, Deserialize)]
struct City {
    name: String,
}

#[derive(Debug, Deserialize)]
struct ForecastItem {
    dt_txt: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
struct AirQualityResponse {
    list: Vec<AirQualityItem>,
}

#[derive(Debug, Deserialize)]
struct AirQualityItem {
    main: AirQualityMain,
    components: Components,
}

#[derive(Debug, Deserialize)]
struct AirQualityMain {
    aqi: i32,
}

#[derive(Debug, Deserialize)]
struct Components {
    pm2_5: f64,
    pm10: f64,
}

async fn get_current_weather(client: &reqwest::Client, api_key: &str) -> Result<(), Box<dyn Error>> {
    println!("Enter ZIP code:");
    let mut zip = String::new();
    io::stdin().read_line(&mut zip)?;
    let zip = zip.trim();

    if !is_valid_zip(zip) {
        println!("Invalid ZIP code. Please enter a 5-digit US ZIP code.");
        return Ok(());
    }

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?zip={},us&appid={}&units=imperial",
        zip, api_key
    );

    match client.get(&url).send().await?.json::<WeatherResponse>().await {
        Ok(response) => {
            println!("\nCurrent Weather for {} ({})", response.name, zip);
            println!("Temperature: {:.1}°F", response.main.temp);
            println!("Humidity: {}%", response.main.humidity);
            println!("Wind Speed: {} mph", response.wind.speed);
            println!("Conditions: {}", response.weather[0].description);
        },
        Err(_) => println!("Unable to fetch weather data for ZIP code {}", zip),
    }
    Ok(())
}

async fn get_forecast(client: &reqwest::Client, api_key: &str) -> Result<(), Box<dyn Error>> {
    println!("Enter ZIP code:");
    let mut zip = String::new();
    io::stdin().read_line(&mut zip)?;
    let zip = zip.trim();

    if !is_valid_zip(zip) {
        println!("Invalid ZIP code. Please enter a 5-digit US ZIP code.");
        return Ok(());
    }

    let url = format!(
        "http://api.openweathermap.org/data/2.5/forecast?zip={},us&appid={}&units=imperial",
        zip, api_key
    );

    match client.get(&url).send().await?.json::<ForecastResponse>().await {
        Ok(response) => {
            println!("\n5-Day Forecast for {} ({})", response.city.name, zip);
            for item in response.list.iter().step_by(8) {  // Get one per day (8 = 24h / 3h intervals)
                println!("\nDate: {}", item.dt_txt);
                println!("Temperature: {:.1}°F", item.main.temp);
                println!("Conditions: {}", item.weather[0].description);
            }
        },
        Err(_) => println!("Unable to fetch forecast data for ZIP code {}", zip),
    }
    Ok(())
}

async fn get_air_quality(client: &reqwest::Client, api_key: &str) -> Result<(), Box<dyn Error>> {
    println!("Enter ZIP code:");
    let mut zip = String::new();
    io::stdin().read_line(&mut zip)?;
    let zip = zip.trim();

    if !is_valid_zip(zip) {
        println!("Invalid ZIP code. Please enter a 5-digit US ZIP code.");
        return Ok(());
    }

    // First get coordinates from ZIP code
    let geo_url = format!(
        "http://api.openweathermap.org/geo/1.0/zip?zip={},us&appid={}",
        zip, api_key
    );
    
    let geo_response = client.get(&geo_url).send().await?;
    let geo_data: serde_json::Value = geo_response.json().await?;
    
    let lat = geo_data["lat"].as_f64().ok_or("Invalid latitude")?;
    let lon = geo_data["lon"].as_f64().ok_or("Invalid longitude")?;
    let name = geo_data["name"].as_str().unwrap_or("Unknown Location");

    let url = format!(
        "http://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );

    match client.get(&url).send().await?.json::<AirQualityResponse>().await {
        Ok(response) => {
            println!("\nAir Quality for {} ({})", name, zip);
            println!("Air Quality Index: {}", response.list[0].main.aqi);
            println!("PM2.5: {:.1} μg/m³", response.list[0].components.pm2_5);
            println!("PM10: {:.1} μg/m³", response.list[0].components.pm10);
        },
        Err(_) => println!("Unable to fetch air quality data for ZIP code {}", zip),
    }
    Ok(())
}

async fn get_weather_alerts(client: &reqwest::Client, api_key: &str) -> Result<(), Box<dyn Error>> {
    println!("Enter ZIP code:");
    let mut zip = String::new();
    io::stdin().read_line(&mut zip)?;
    let zip = zip.trim();

    if !is_valid_zip(zip) {
        println!("Invalid ZIP code. Please enter a 5-digit US ZIP code.");
        return Ok(());
    }

    let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?zip={},us&appid={}&units=imperial",
        zip, api_key
    );

    match client.get(&url).send().await?.json::<serde_json::Value>().await {
        Ok(json) => {
            let location_name = json["name"].as_str().unwrap_or("Unknown Location");
            println!("\nWeather Alerts for {} ({})", location_name, zip);
            if let Some(alerts) = json.get("alerts") {
                println!("Alerts: {}", alerts);
            } else {
                println!("No active weather alerts");
            }
        },
        Err(_) => println!("Unable to fetch weather alerts for ZIP code {}", zip),
    }
    Ok(())
}

async fn compare_locations(client: &reqwest::Client, api_key: &str) -> Result<(), Box<dyn Error>> {
    println!("Enter ZIP codes (comma-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let zip_codes: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

    println!("\nWeather Comparison:");
    for zip in zip_codes {
        if !is_valid_zip(zip) {
            println!("\n{}: Invalid ZIP code (skipping)", zip);
            continue;
        }

        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?zip={},us&appid={}&units=imperial",
            zip, api_key
        );

        match client.get(&url).send().await?.json::<WeatherResponse>().await {
            Ok(weather) => {
                println!("\n{} ({})", weather.name, zip);
                println!("Temperature: {:.1}°F", weather.main.temp);
                println!("Conditions: {}", weather.weather[0].description);
            }
            Err(_) => println!("\n{}: Unable to fetch weather data", zip),
        }
    }
    Ok(())
}
