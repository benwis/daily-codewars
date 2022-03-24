use chrono::Duration;

fn is_plural(num: i64) -> String{
    if num > 1{
        "s".to_string()
    }
    else{
        "".to_string()
    }
}
fn parse_seperators(str: &str) -> String{
    let mut response = str.to_owned();
    // Count number of * in response string. If 1, delete it. If 2, replace with comma and replace last one with ' and '
    let seperators: usize = response.chars().filter(|&x| x == '*').count();
    if seperators == 0{
    }
    else if seperators == 1{
        response = response.replace("*", "");
    }
    else if seperators == 2{
        response = response.replacen("*", " and ", 1);
        response = response.replace("*", "");
        }
    else if seperators > 2{
        response = response.replacen("*", ", ",seperators-2);
        response = response.replacen("*", " and ", 1);
        response = response.replace("*", "");
    }
    else{
        panic!("This shouldn't be possible!")
    }
    response
}
fn format_duration(seconds: u64) -> String {
    // let mut years: u64 = 0;
    // let mut days: u64 = 0;
    // let mut hours: u64 = 0;
    // let mut minutes: u64 = 0;
    // let mut out_seconds: u64 = 0;
    let mut response: String = String::from("");
    
    if seconds == 0{
        return "now".to_string();
    }
    
    let mut duration = Duration::seconds(seconds.try_into().unwrap());
    //Calculate years
    if duration.num_weeks() > 52{
        //Calculate number of years
        let years: i64 = duration.num_days() / 365;
        duration = duration - Duration::days(365*years);
        response.push_str(&format!("{} year{}*", &years, is_plural(years)));
    }
    if duration.num_days() >= 1{
        let days: i64 = duration.num_days();
        duration = duration -  Duration::days(days);
        response.push_str(&format!("{} day{}*", &days, is_plural(days)));

    }
    if duration.num_hours() >= 1{
        let hours: i64 = duration.num_hours();
        duration = duration - Duration::hours(hours);
        response.push_str(&format!("{} hour{}*", &hours, is_plural(hours)));

    }
    if duration.num_minutes() >= 1{
        let minutes: i64 = duration.num_minutes();
        duration = duration - Duration::minutes(minutes);
        response.push_str(&format!("{} minute{}*", &minutes, is_plural(minutes)));

    }
    if duration.num_seconds() > 0{
        let out_seconds: i64 = duration.num_seconds();
        response.push_str(&format!("{} second{}*", &out_seconds, is_plural(out_seconds)));

    }
    parse_seperators(&response)
}

#[test]
fn test_basic() {
    assert_eq!(format_duration(1), "1 second");
    assert_eq!(format_duration(62), "1 minute and 2 seconds");
    assert_eq!(format_duration(120), "2 minutes");
    assert_eq!(format_duration(3600), "1 hour");
    assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
}
}
