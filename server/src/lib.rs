use chrono::{NaiveDate, Weekday};
use rocket::serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub task_type: TaskType,
    pub date: Option<String>,
    pub week_days: Option<Vec<Weekday>>,
    pub name: String,
}

impl Task {
    pub fn to_string(&self) -> String {
        format!(
            "{}::{}::{}::{}::{}",
            self.id,
            self.name,
            match self.task_type {
                TaskType::HABIT => "HABIT",
                TaskType::TODO => "TODO",
            },
            match &self.date {
                Some(date) => date,
                None => "null",
            },
            match &self.week_days {
                Some(week_days) => week_days
                    .iter()
                    .map(|day| match day {
                        Weekday::Mon => "MON".to_string(),
                        Weekday::Tue => "TUE".to_string(),
                        Weekday::Wed => "WED".to_string(),
                        Weekday::Thu => "THU".to_string(),
                        Weekday::Fri => "FRI".to_string(),
                        Weekday::Sat => "SAT".to_string(),
                        Weekday::Sun => "SUN".to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join(","),
                None => "null".to_string(),
            }
        )
    }

    pub fn string_to_task(data: String) -> Task {
        let task_data: Vec<&str> = data.split("::").collect();
        let id = task_data[0].to_string();
        let name = task_data[1].to_string();
        let task_type = match task_data[2] {
            "HABIT" => TaskType::HABIT,
            "TODO" => TaskType::TODO,
            _ => {
                panic!("Invalid task type value")
            }
        };

        let date = match task_data[3] {
            "null" => None,
            _ => Some(
                NaiveDate::parse_from_str(task_data[3], "%Y-%m-%d")
                    .unwrap()
                    .to_string(),
            ),
        };

        let mut parsed_week_days: Vec<Weekday> = vec![];
        if !task_data[4].contains("null") {
            let week_days: Vec<&str> = task_data[4].split(",").collect();

            for week_day in week_days {
                match week_day {
                    "MON" => parsed_week_days.push(Weekday::Mon),
                    "TUE" => parsed_week_days.push(Weekday::Tue),
                    "WED" => parsed_week_days.push(Weekday::Wed),
                    "THU" => parsed_week_days.push(Weekday::Thu),
                    "FRI" => parsed_week_days.push(Weekday::Fri),
                    "SAT" => parsed_week_days.push(Weekday::Sat),
                    "SUN" => parsed_week_days.push(Weekday::Sun),
                    _ => {
                        panic!("Invalid week days value")
                    }
                };
            }
        }

        Task {
            id,
            name,
            task_type,
            week_days: match parsed_week_days.len() {
                d if d > 0 => Some(parsed_week_days),
                _ => None,
            },
            // ***SEARCH DONE VALUE IN stats.txt FILE AND SEND HERE***
            date: date,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum TaskType {
    TODO,
    HABIT,
}

pub struct ManageDatabase {}

impl ManageDatabase {
    pub fn read_data() -> Vec<Task> {
        let input = File::open("tasks.txt").unwrap();
        let buffered = BufReader::new(input);
        let mut lines: Vec<String> = vec![];

        for line in buffered.lines() {
            lines.push(line.unwrap().to_string());
        }

        let mut tasks: Vec<Task> = vec![];

        for line in lines.clone() {
            if line.contains("id::name::type::date::week,days") {
                continue;
            }
            tasks.push(Task::string_to_task(line));
        }

        tasks
    }

    pub fn write_data(data: String) {
        let mut output = File::create("tasks.txt").unwrap();
        write!(output, "{}", data).unwrap();
    }
}
