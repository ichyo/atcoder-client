fn get_standings_url(id: &str) -> String {
    format!("https://beta.atcoder.jp/contests/{}/standings/json", id)
}

pub fn get_standings(id: &str) -> String {
    let url = get_standings_url(id);
    url
}
