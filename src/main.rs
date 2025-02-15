use plotters::prelude::Circle;
use std::{fs};
use chrono::{DateTime, Duration, NaiveDateTime, NaiveTime, Timelike, Utc};
use walkdir::*;
use regex::Regex;
use plotters::prelude::*;

const OUT_FILE_NAME: &str = "scatterplot.png";

fn main() {
    let timestamps =  get_all_timestamps("testdata"); // TODO: replace with user input from gui
    plot_datetime_scatter(&timestamps, "chart.png").unwrap();
}

fn get_all_timestamps(messages_folder: &str) -> Vec<DateTime<Utc>>{
    let ts_format = "%Y-%m-%d %H:%M:%S";
    let re = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})").unwrap(); // ISO 8601 format. Probably faster than parsing json
    let mut timestamps: Vec<DateTime<Utc>> = Vec::new();
    let walker = WalkDir::new(messages_folder).into_iter();
    walker.for_each(|entry| {
        let entry = entry.unwrap();
        if entry
            .file_name()
            .to_str()
            .map_or(false, |s| s.eq("messages.json")) // only parse relevant files
        {
            let content = fs::read_to_string(entry.path()).unwrap();
            for timestamp in re.find_iter(&content) {
                // timestamps are implied to be UTC but don't include a TZ specifier
                // so we need to parse as NaiveDateTime first and convert to regular Datetime with and_utc()
                timestamps.push(NaiveDateTime::parse_from_str(timestamp.as_str(), ts_format).unwrap().and_utc());
            }
        }
    });
    timestamps
}

fn plot_datetime_scatter(data: &Vec<DateTime<Utc>>, output_file: &str) -> Result<()> {
    let root = BitMapBackend::new(output_file, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let x_min = *data.iter().min().unwrap();
    let x_max = *data.iter().max().unwrap();

    let y_min = Duration::seconds(0);
    let y_max = Duration::seconds(86400); // seconds in a day

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .caption("Time", ("sans-serif", 30.0).into_font())
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    chart.configure_mesh()
        .light_line_style(&WHITE)
        .y_label_formatter(&|y| format!("{:02}:{:02}", y.num_minutes(), y.num_seconds() % 60))
        .x_label_formatter(&|x| x.naive_local().to_string())
        .draw()
        .unwrap();

    chart.draw_series(
        data.iter()
            .map(|x|
                // why doesn't it work man just get off my fucking back please for 5 seconds just kill me please
                Circle::new((x.date_naive(), Duration::seconds(x.time().num_seconds_from_midnight() as i64)), 5, BLUE)
            ),
    ).unwrap();

    Ok(())
}
