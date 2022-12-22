use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use chrono::{DateTime, Local};
use clap::{App, Arg};
use colored::*;
use fern::colors::{Color, ColoredLevelConfig};
use prettytable::{cell, row, table};
use regex::Regex;
use tar::Archive;

const NGINX_LOG_FORMAT: &str = r#"^(?P<remote_addr>[\d\.]+) (?P<remote_user>\S+) (?P<request_time>\[[^\]]+\]) (?P<request>\"[^\"]+\") (?P<status>\d+) (?P<body_bytes_sent>\d+) (?P<http_referer>\"[^\"]+\") (?P<http_user_agent>\"[^\"]+\")"#;

#[derive(Debug)]
struct NginxLog {
    remote_addr: String,
    remote_user: String,
    request_time: DateTime<Local>,
    request: String,
    status: u16,
    body_bytes_sent: u64,
    http_referer: String,
    http_user_agent: String,
}

fn parse_nginx_log_line(line: &str) -> Option<NginxLog> {
    let re = Regex::new(NGINX_LOG_FORMAT).unwrap();
    let captures = match re.captures(line) {
        Some(captures) => captures,
        None => return None,
    };
    let request_time = DateTime::parse_from_str(&captures["request_time"], "%d/%b/%Y:%H:%M:%S %z")
        .unwrap();
    let status = captures["status"].parse::<u16>().unwrap();
    let body_bytes_sent = captures["body_bytes_sent"].parse::<u64>().unwrap();
    Some(NginxLog {
        remote_addr: captures["remote_addr"].to_string(),
        remote_user: captures["remote_user"].to_string(),
        request_time,
        request: captures["request"].to_string(),
        status,
        body_bytes_sent,
        http_referer: captures["http_referer"].to_string(),
        http_user_agent: captures["http_user_agent"].to_string(),
    })
}

fn print_logs(logs: &[NginxLog]) {
    let mut table = table!([bFg -> "Remote Address", "Remote User", "Request Time", "Request", "Status", "Body Bytes Sent", "HTTP Referer", "HTTP User Agent"]);
    for log in logs {
        table.add_row(row![
            log.remote_addr,
            log.remote_user,
            log.request_time.to_string(),
            log.request,
            log.status,
            log.body_bytes_sent,
            log.http_referer,
            log.http_user_agent,
        ]);
    }
    table.printstd();
}


fn read_nginx_logs<P: AsRef<Path>>(path: P) -> Result<Vec<NginxLog>, Box<dyn Error>> {
    let path = path.as_ref();
    let logs = if path.is_dir() {
        let mut logs = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "log" || ext == "gz" {
                    if ext == "gz" {
                        let temp_dir = tempfile::tempdir()?;
                        let temp_path = temp_dir.path().join(path.file_name().unwrap());
                        extract_tar(&path, &temp_path)?;
                        logs.append(&mut read_nginx_logs(temp_path)?);
                        fs::remove_file(temp_path)?;
                    } else {
                        logs.append(&mut read_nginx_logs(path)?);
                    }
                }
            }
        }
        logs
    } else {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut logs = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if let Some(log) = parse_nginx_log_line(&line) {
                logs.push(log);
            }
        }
        logs
    };
    Ok(logs)
}

fn main() -> Result<(), Box<dyn Error>> {
    let logs = read_nginx_logs("/path/to/logs")?;
    print_logs(&logs);
    Ok(())
}
