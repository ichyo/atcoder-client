#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Competition {
    pub is_rated: bool,
    pub place: u64,
    pub new_rating: i32,
    pub performance: i32,
    pub inner_performance: i32,
    pub contest_screen_name: String,
    pub contest_name: String,
}

impl Competition {
    pub fn contest_id(&self) -> Option<&str> {
        self.contest_screen_name.split('.').next()
    }
}

fn get_history_url(id: &str) -> String {
    format!("https://beta.atcoder.jp/users/{id}/history/json", id = id)
}

pub fn get_history(id: &str) -> Result<Vec<Competition>, reqwest::Error> {
    let url = get_history_url(id);
    let mut response = reqwest::get(&url)?;
    let history = response.json()?;
    Ok(history)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history() {
        let _ = get_history("ichyo").unwrap();
    }

    #[test]
    fn test_contest_id() {
        let c = Competition {
            is_rated: false,
            place: 10000,
            new_rating: 1500,
            performance: 1500,
            inner_performance: 1500,
            contest_screen_name: "mujin-pc-2018.contest.atcoder.jp".to_string(),
            contest_name: "Mujin Programming Contest Challenge 2018".to_string(),
        };
        assert_eq!("mujin-pc-2018", c.contest_id().unwrap());
    }
}
