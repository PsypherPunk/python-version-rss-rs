use ::python_version_rss::*;
use std::io::Write;

fn main() {
    let response = get_json_feed().expect("JSON feed retrieval failed");
    let releases = get_releases_from_response(response).expect("parsing releases failed");
    let mut channel = get_channel().expect("building RSS channel failed");
    let mut items = get_rss_items(releases);

    items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));
    channel.items = items;

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("release.rss")
        .expect("output file creation failed");

    file.write_all(channel.to_string().as_bytes())
        .expect("writing RSS failed");
}
