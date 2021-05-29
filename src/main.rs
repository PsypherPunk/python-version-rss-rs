use ::python_version_rss::*;
use std::io::Write;

fn main() {
    let response = get_json_feed().expect("JSON feed retrieval failed");

    let mut releases = get_releases_from_response(response).expect("parsing releases failed");
    get_sorted_releases(&mut releases);

    let mut channel = get_channel().expect("building RSS channel failed");
    let items = get_rss_items(releases);
    channel.items = items;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open("release.rss")
        .expect("output file creation failed");

    file.write_all(channel.to_string().as_bytes())
        .expect("writing RSS failed");
}
