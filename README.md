# Weather App

A command-line weather application built in Rust that provides comprehensive weather information using the OpenWeatherMap API.

## Features

- **Current Weather**: Get real-time temperature, humidity, wind speed, and conditions for any US ZIP code
- **5-Day Forecast**: View weather predictions for the next 5 days
- **Air Quality**: Check air quality index and particulate matter measurements
- **Weather Alerts**: Stay informed about active weather warnings and advisories
- **Location Comparison**: Compare weather conditions across multiple locations simultaneously

## Prerequisites

- Rust (1.56.0 or later)
- Cargo (included with Rust)
- OpenWeatherMap API key (free tier available at [openweathermap.org](https://openweathermap.org/api))

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/weather_app.git
   cd weather_app
   ```

2. Update the API key in `src/main.rs`:
   ```rust
   let api_key = "your_api_key_here"; // Replace with your API key
   ```

3. Build the application:
   ```
   cargo build --release
   ```

## Usage

Run the application:
```
cargo run --release
```

Follow the interactive menu to select from available options:
1. Current Weather
2. 5-Day Forecast
3. Air Quality
4. Weather Alerts
5. Compare Multiple Locations
6. Exit

When prompted, enter a valid 5-digit US ZIP code to get weather information for that location.

For the location comparison feature, enter multiple ZIP codes separated by commas (e.g., `10001,90210,60601`).

## Dependencies

- [reqwest](https://crates.io/crates/reqwest) - HTTP client for API requests
- [serde](https://crates.io/crates/serde) - Serialization/deserialization framework
- [tokio](https://crates.io/crates/tokio) - Asynchronous runtime
- [chrono](https://crates.io/crates/chrono) - Date and time handling

## API Reference

This application uses the following OpenWeatherMap API endpoints:
- Current Weather: `/data/2.5/weather`
- 5-Day Forecast: `/data/2.5/forecast`
- Air Quality: `/data/2.5/air_pollution`
- Geocoding: `/geo/1.0/zip`

For more information about these endpoints, visit the [OpenWeatherMap API documentation](https://openweathermap.org/api).

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Weather data provided by [OpenWeatherMap](https://openweathermap.org/)
- Built with [Rust](https://www.rust-lang.org/)
