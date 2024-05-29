use serde::Deserialize;

#[derive(Deserialize)]
struct SunriseSunsetIoResponse {
    results: ResponseResults,
}

#[derive(Deserialize)]
struct ResponseResults {
    #[serde(with = "time::serde::rfc3339")]
    sunset: time::OffsetDateTime,
}

pub async fn get_sunset_time() -> Result<time::OffsetDateTime, reqwest::Error> {
    const DUBLIN_COORDS: (&str, &str) = ("53.345727", "-6.269727");
    const TIMEZONE: &str = "Europe/Dublin";
    let url = format!(
        "http://api.sunrise-sunset.org/json?lat={}&lng={}&formatted=0&tzid={}",
        DUBLIN_COORDS.0, DUBLIN_COORDS.1, TIMEZONE
    );

    let resp = reqwest::get(url)
        .await?
        .json::<SunriseSunsetIoResponse>()
        .await?;

    Ok(resp.results.sunset)
}
