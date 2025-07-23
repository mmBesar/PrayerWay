use std::collections::HashMap;
use std::fs::{metadata, read_to_string, File};
use std::io::Write;
use std::process::exit;
use std::time::{Duration, SystemTime};

use chrono::prelude::*;
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::{json, Value};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, help = "Specify a city")]
    city: Option<String>,
    #[arg(long, help = "Specify a country")]
    country: Option<String>,
    #[arg(long, help = "Specify a calculation method (see https://aladhan.com/calculation-methods)")]
    method: Option<String>,
    #[arg(long, help = "Display calendar in Arabic format")]
    ar: bool,
    #[arg(long, help = "Display time in 12-hour AM/PM format")]
    am_pm: bool,
    #[arg(long, help = "Specify a custom notification audio file")]
    audio: Option<String>,
    #[arg(long, help = "Notify before prayer in minutes (default: 10)")]
    notify: Option<u32>,
}

const DEFAULT_ICON: &str = "󱠧";

fn main() {
    let args = Args::parse();

    if args.city.is_none() || args.country.is_none() {
        eprintln!("Missing required arguments. Usage:");
        eprintln!(
            "--city <City> --country <Country> [--method <Method>] [--ar] [--am_pm] [--audio <file>] [--notify <minutes>]"
        );
        exit(1);
    }

    let dt = Local::now();
    let prayer_url = format!(
        "http://api.aladhan.com/v1/timingsByCity/{}?city={}&country={}&method={}",
        dt.format("%d-%m-%Y"),
        args.city.as_ref().unwrap(),
        args.country.as_ref().unwrap(),
        args.method.as_ref().unwrap_or(&String::default())
    );

    let cachefile = format!("/tmp/prayerbar-{}.json", args.city.as_ref().unwrap());
    let client = Client::new();
    let times = get_prayer_times(&client, &prayer_url, &cachefile);

    let data = parse_prayer_times(times, &args);
    let json_data = json!(data);

    println!("{}", json_data);
}

fn get_prayer_times(client: &Client, prayer_url: &str, cachefile: &str) -> Value {
    let is_cache_file_recent = if let Ok(metadata) = metadata(&cachefile) {
        let five_hours_ago = SystemTime::now() - Duration::from_secs(10800);
        metadata.modified().map_or(false, |mod_time| mod_time > five_hours_ago)
    } else {
        false
    };

    if is_cache_file_recent {
        let json_str = read_to_string(&cachefile).expect("Unable to read cache file");
        serde_json::from_str::<Value>(&json_str).expect("Unable to parse cache file")
    } else {
        let response = client.get(prayer_url).send().expect("Error connecting to API");
        let times = response.json::<Value>().expect("Unable to parse response");

        let mut file = File::create(cachefile).expect("Unable to create cache file");
        file.write_all(serde_json::to_string_pretty(&times).unwrap().as_bytes())
            .expect("Unable to write cache file");

        times
    }
}

fn parse_prayer_times<'a>(times: Value, args: &Args) -> HashMap<&'a str, String> {
    let dt = Local::now();
    let mut data = HashMap::new();

    // Filtered prayer names
    let prayer_names = [
        ("Fajr", "الفجر"),
        ("Sunrise", "الشروق"),
        ("Dhuhr", "الظهر"),
        ("Asr", "العصر"),
        ("Maghrib", "المغرب"),
        ("Isha", "العشاء"),
        ("Last Third of the Night", "الثلث الأخير من الليل"),
    ];

    let default_city = "City".to_string();
    let city_name = args.city.as_ref().unwrap_or(&default_city);

    let hijri_date = format_hijri_date(&times, if args.ar { "ar" } else { "en" });

    let tooltip_header = if args.ar {
        format!("مواقيت الصلاة في {}", city_name)
    } else {
        format!("Prayer Times in {}", city_name)
    };

    let mut tooltip = format!("{}\n\n{}\n\n", hijri_date, tooltip_header);

    let prayer_times_map = times["data"]["timings"]
        .as_object()
        .expect("Prayer timings not available");

    let mut prayer_data: Vec<(&str, DateTime<FixedOffset>)> = Vec::new();
    for (prayer_name, prayer_time) in prayer_times_map.iter() {
        if prayer_names.iter().any(|(key, _)| *key == prayer_name) {
            let prayer_time_value = prayer_time.as_str().expect("Prayer time not available");
            let date_time_str = format!(
                "{} {} {}",
                dt.format("%Y-%m-%d"),
                prayer_time_value,
                dt.format("%z")
            );
            let date_time = DateTime::parse_from_str(&date_time_str, "%Y-%m-%d %H:%M %z")
                .expect("Unable to parse date time");
            prayer_data.push((prayer_name.as_str(), date_time));
        }
    }

    prayer_data.sort_by(|a, b| a.1.cmp(&b.1));

    // Current and Next Prayer
    let current_prayer = prayer_data
        .iter()
        .rev()
        .find(|(_, time)| *time <= dt.with_timezone(&FixedOffset::east_opt(0).unwrap()));
    let next_prayer = prayer_data
        .iter()
        .find(|(_, time)| *time > dt.with_timezone(&FixedOffset::east_opt(0).unwrap()));

    if let Some((current_prayer_name, current_prayer_time)) = current_prayer {
        let current_display = if args.ar {
            format!("الآن {} {:<20}", translate_prayer_name(current_prayer_name, &prayer_names), format_time(current_prayer_time, args))
        } else {
            format!("الآن {} {:<20}", translate_prayer_name(current_prayer_name, &prayer_names), format_time(current_prayer_time, args))
        };
        tooltip.push_str(&format!("{}\n", current_display));
    }

    if let Some((next_prayer_name, next_prayer_time)) = next_prayer {
        let countdown = (*next_prayer_time - dt.with_timezone(next_prayer_time.offset()))
            .to_std()
            .unwrap();
        let countdown_str = if args.ar {
            format!(
                "بعد {} ساعة و {} دقيقة",
                countdown.as_secs() / 3600,
                (countdown.as_secs() % 3600) / 60
            )
        } else {
            format!(
                "in {}h {}m",
                countdown.as_secs() / 3600,
                (countdown.as_secs() % 3600) / 60
            )
        };

        let next_display = format!(
            "{} {}",
            translate_prayer_name(next_prayer_name, &prayer_names),
            countdown_str
        );
        tooltip.push_str(&format!("{}\n\n", next_display));
    }

    // Add a blank line for better separation
    tooltip.push('\n');

    // Prayer Times
    for (prayer_name, prayer_time) in &prayer_data {
        let formatted_time = format_time(prayer_time, args);
        let translated_name = translate_prayer_name(prayer_name, &prayer_names);
        tooltip.push_str(&format!("{:<20} {}\n", translated_name, formatted_time));
    }

    data.insert("text", DEFAULT_ICON.to_string());
    data.insert("tooltip", tooltip);
    data
}

fn translate_prayer_name<'a>(name: &'a str, mapping: &[(&'a str, &'a str)]) -> &'a str {
    mapping
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, value)| *value)
        .unwrap_or(name)
}

fn format_hijri_date(times: &Value, language: &str) -> String {
    let weekday = times["data"]["date"]["hijri"]["weekday"][language]
        .as_str()
        .unwrap_or("N/A");
    let day = times["data"]["date"]["hijri"]["day"]
        .as_str()
        .unwrap_or("N/A");
    let month = times["data"]["date"]["hijri"]["month"][language]
        .as_str()
        .unwrap_or("N/A");
    let year = times["data"]["date"]["hijri"]["year"]
        .as_str()
        .unwrap_or("N/A");
    format!("{} {} {} {}", weekday, day, month, year)
}

fn format_time(time: &DateTime<FixedOffset>, args: &Args) -> String {
    let format_str = if args.am_pm {
        if args.ar {
            time.format("%I:%M %p")
                .to_string()
                .replace("AM", "ص")
                .replace("PM", "م")
        } else {
            time.format("%I:%M %p").to_string()
        }
    } else {
        time.format("%H:%M").to_string()
    };
    format_str
}
