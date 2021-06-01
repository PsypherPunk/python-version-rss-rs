# `python-version-rss-rs`

## What?

Generates an RSS feed of Python releases, which should be visible
[here](https://raw.githubusercontent.com/PsypherPunk/python-version-rss-rs/main/release.rss).

## Why?

I wanted an RSS feed of Python versions but quite simply, couldn't find one.
`python.org` does, however,
[publish](https://www.python.org/api/v2/downloads/release/) a list of every
version ever released. So why not turn that into an RSS feed?

## How?

This relies on GitHub Actions to read the JSON feed referenced above and
convert this into an RSS feed, which we then `git commit` back into the repo.

It's it Rust which, in hindsight, wasn't the right choice: the compile times
during the GitHub Action negate any benefit. That said, I hadn't had the
opportunity to do some of the things in Rust I played with during development
so there's that.
