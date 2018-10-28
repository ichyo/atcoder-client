use std::collections::HashMap;

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Standings {
    fixed: bool,
    task_info: Vec<TaskInfo>,
    standings_data: Vec<StandingsData>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TaskInfo {
    assignment: String,
    task_name: String,
    task_screen_name: String,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StandingsData {
    rank: u64,
    user_name: String,
    user_screen_name: String,
    task_results: HashMap<String, TaskResult>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TaskResult {
    score: i64,
}

fn get_standings_url(id: &str) -> String {
    format!(
        "https://beta.atcoder.jp/contests/{id}/standings/json",
        id = id
    )
}

pub fn get_standings(id: &str) -> Result<Standings, reqwest::Error> {
    let url = get_standings_url(id);
    let mut response = reqwest::get(&url)?;
    let standings = response.json()?;
    Ok(standings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standings() {
        let _ = get_standings("tenka1-2018").unwrap();
    }
}
