# Hugo Lunr

A utility to produce a Lunr index from a static Hugo site. Currently only supports `.md` and TOML front matter.

Usage:
`hugo_lunr example/blog/content example/blog/static/index.json`

Defaults to `./content` for the content directory and `./static/index.json` for the index output.