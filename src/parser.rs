pub fn find_id_in_base_website_source(body: &str) -> Option<i64> {
    for line in body.lines() {
        if !line.contains("episodeData") {
            continue;
        }
        if !line.contains("data: ") {
            continue;
        }

        let id = line.split("id:").nth(1)?;
        let id = id.split(",").next()?;

        return Some(
            id.trim()
                .to_string()
                .parse()
                .map_err(|e| {
                    eprintln!("Failed to parse ID: {}", e);
                    std::process::exit(1);
                })
                .unwrap(),
        );
    }
    None
}
