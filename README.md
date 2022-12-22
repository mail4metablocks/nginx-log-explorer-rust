# Nginx Log Explorer

A Rust crate for reading and analyzing Nginx logs.

## Features

    Reads logs from a file or directory, including gzipped log files.
    Filters logs by dates, status, referers, and paths.
    Performs trend analysis on the logs.
    Prints the results in a table to the console.


### Implementation

The crate includes the following functions:
 ###read_nginx_logs

This function reads Nginx logs from a given file or directory. If the path is a directory, it recursively searches for log files within the directory and its subdirectories. If it finds a gzipped log file, it extracts it to a temporary location and reads the logs from there. It returns a vector of NginxLog structures representing the logs that were read.
###parse_nginx_log_line

This function parses a single line of an Nginx log file and returns a NginxLog structure representing the log. It uses a regular expression to extract the relevant information from the log line.
###filter_logs

This function filters a vector of logs based on the given start and end dates, status, referer, and path. It returns a new vector containing only the logs that match the given criteria.
###trend_analysis

This function performs trend analysis on a vector of logs by counting the number of logs per day and returning a map from dates (in the format "YYYY-MM-DD") to the count of logs on that day.
###print_logs

This function prints a table of the given logs to the console using the prettytable crate.

The crate also includes a command-line interface that allows you to use these functions to analyze Nginx logs from the command line. You can specify the path to the logs, the start and end dates, the status, referer, and path to filter the logs by, and the trend analysis mode. The results are printed to the console in a table.
