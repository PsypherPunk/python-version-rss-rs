use chrono::{DateTime, Utc};
use rss::{Channel, ChannelBuilder, Guid, Item};
use serde::Deserialize;
use ureq::Response;

const PYTHON_RELEASE_URL: &str = "https://www.python.org/api/v2/downloads/release/";

#[derive(Debug, Deserialize)]
pub struct Release {
    name: String,
    slug: String,
    version: usize,
    is_published: bool,
    is_latest: bool,
    release_date: String,
    pre_release: bool,
    release_page: Option<String>,
    release_notes_url: String,
    show_on_download_page: bool,
    resource_uri: String,
}

pub fn get_json_feed() -> Result<Response, ureq::Error> {
    ureq::get(PYTHON_RELEASE_URL).call()
}

pub fn get_releases_from_response(response: Response) -> Result<Vec<Release>, std::io::Error> {
    response.into_json()
}

pub fn get_channel() -> Result<Channel, String> {
    ChannelBuilder::default()
        .title("Python Releases")
        .link(PYTHON_RELEASE_URL)
        .description("Feed of all Python releases.")
        .build()
}

fn get_rfc822_date(date: &str) -> String {
    date.parse::<DateTime<Utc>>()
        .expect("invalid date")
        .to_rfc2822()
}

pub fn get_sorted_releases(releases: &mut Vec<Release>) {
    releases.sort_by(|a, b| {
        b.release_date
            .parse::<DateTime<Utc>>()
            .expect("invalid date")
            .cmp(
                &a.release_date
                    .parse::<DateTime<Utc>>()
                    .expect("invalid date"),
            )
    });
}

fn get_guid(slug: &str) -> Guid {
    Guid {
        value: slug.to_string(),
        permalink: false,
    }
}

pub fn get_rss_items(releases: Vec<Release>) -> Vec<Item> {
    releases
        .iter()
        .map(|release| Item {
            title: Some(release.name.to_owned()),
            link: Some(release.resource_uri.to_owned()),
            description: Some(release.release_notes_url.to_owned()),
            author: None,
            categories: Vec::new(),
            comments: None,
            enclosure: None,
            guid: Some(get_guid(&release.slug)),
            pub_date: Some(get_rfc822_date(&release.release_date)),
            source: None,
            content: None,
            extensions: Default::default(),
            itunes_ext: None,
            dublin_core_ext: None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_feed() {
        let feed = get_json_feed();

        assert!(feed.is_ok());
        assert!(feed.unwrap().into_string().is_ok());
    }

    #[test]
    fn test_get_releases() {
        let response = get_json_feed().unwrap();
        let releases = get_releases_from_response(response);

        assert!(releases.is_ok());
    }

    #[test]
    fn test_get_rfc822() {
        assert_eq!(
            "Fri, 02 Apr 2021 17:32:13 +0000",
            get_rfc822_date("2021-04-02T17:32:13Z"),
        );
    }
}
