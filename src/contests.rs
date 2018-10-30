use std::collections::HashMap;

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Standings {
    pub fixed: bool,
    pub task_info: Vec<TaskInfo>,
    pub standings_data: Vec<StandingsData>,
}

impl Standings {
    pub fn task_ids(&self) -> Vec<String> {
        self.task_info.iter().map(|t| t.id().to_string()).collect()
    }

    pub fn standings(&self) -> &Vec<StandingsData> {
        &self.standings_data
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TaskInfo {
    pub assignment: String,
    pub task_name: String,
    pub task_screen_name: String,
}

impl TaskInfo {
    pub fn id(&self) -> &str {
        &self.task_screen_name
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StandingsData {
    pub rank: u64,
    pub user_name: String,
    pub user_screen_name: String,
    pub is_rated: bool,
    pub task_results: HashMap<String, TaskResult>,
    pub old_rating: i32,
    pub rating: i32,
}

impl StandingsData {
    pub fn result(&self, id: &str) -> Option<&TaskResult> {
        self.task_results.get(id)
    }

    pub fn user_id(&self) -> &str {
        &self.user_screen_name
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TaskResult {
    pub score: i64,
    pub status: i16,
}

impl TaskResult {
    pub fn is_solved(&self) -> bool {
        self.status == 1
    }
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
