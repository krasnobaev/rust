fn format_duration(seconds: u64) -> String {
  if seconds == 0 { return (&"now").to_string() }

  let sec = seconds % 60;
  let min = (seconds / 60) % 60;
  let hrs = (seconds / 3600) % 24;
  let dys = (seconds / 3600 / 24) % 365;
  let yrs = seconds / 3600 / 24 / 365;

  let mut res: Vec<_> = vec![];
  if yrs > 1 {
    res.push(format!("{} years", yrs));
  } else if yrs == 1 {
    res.push(format!("{} year", yrs));
  }

  if dys > 1 {
    res.push(format!("{} days", dys));
  } else if dys == 1 {
    res.push(format!("{} day", dys));
  }

  if hrs > 1 {
    res.push(format!("{} hours", hrs));
  } else if hrs == 1 {
    res.push(format!("{} hour", hrs));
  }

  if min > 1 {
    res.push(format!("{} minutes", min));
  } else if min == 1 {
    res.push(format!("{} minute", min));
  }

  if sec > 1 {
    res.push(format!("{} seconds", sec));
  } else if sec == 1 {
    res.push(format!("{} second", sec));
  }

  match res.len() {
    1 => res[0].to_string(),
    2 => format!("{} and {}", res[0], res[1]),
    _ => format!("{} and {}", res[0..res.len()-1].join(", "), res[res.len()-1])
  }

}
