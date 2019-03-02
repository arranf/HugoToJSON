# HugoToJSON

[![Build Status](https://travis-ci.com/arranf/HugoToJSON.svg?branch=master)](https://travis-ci.com/arranf/HugoToJSON)

A utility to produce a JSON representation of the key front matter and contents of Hugo documents. It's main intent is to produce JSON to be used by [Lunr](https://lunrjs.com/) (and [Lunr-like](http://elasticlunr.com/) packages) to support search on a static Hugo site. It's designed to be a fast and modern alternative to the now unsupported [hugo_lunr Node tool](https://www.npmjs.com/package/hugo-lunr).

Pull requests are welcome. A list of goals and work to be done is available in `ToDo.txt`.

It currently supports `.md` files and both YAML and TOML front matter.

## Using
`hugo_to_json HUGO_CONTENT_DIRECTORY -o OUTPUT_LOCATION`

Example usage is shown below.
`hugo_to_json example/blog/content -o example/blog/static/index.json`

Defaults to `./content` for the content directory and stdout for the index output.

## Fetching the Latest Version

If you want to use the latest version of this tool as part of a CI build process the following script should work.

```bash
#!/usr/bin/env bash
set -e

# Based on
#https://blog.markvincze.com/download-artifacts-from-a-latest-github-release-in-sh-and-powershell/

LATEST_RELEASE=$(curl -L -s -H 'Accept: application/json' https://github.com/arranf/HugoLunr/releases/latest)
# The releases are returned in the format {"id":3622206,"tag_name":"hello-1.0.0.11",...}, we have to extract the tag_name.
LATEST_VERSION=$(echo $LATEST_RELEASE | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')
ARTIFACT_URL="https://github.com/arranf/HugoToJSON/releases/download/$LATEST_VERSION/hugo_to_json"
INSTALL_DIRECTORY="."
INSTALL_NAME="hugo_to_json"
DOWNLOAD_FILE="$INSTALL_DIRECTORY/$INSTALL_NAME"

 echo "Fetching $ARTIFACT_URL.."
if test -x "$(command -v curl)"; then
    code=$(curl -s -w '%{http_code}' -L "$ARTIFACT_URL" -o "$DOWNLOAD_FILE")
elif test -x "$(command -v wget)"; then
    code=$(wget -q -O "$DOWNLOAD_FILE" --server-response "$ARTIFACT_URL" 2>&1 | awk '/^  HTTP/{print $2}' | tail -1)
else
    echo "Neither curl nor wget was available to perform http requests."
    exit 1
fi

if [ "$code" != 200 ]; then
    echo "Request failed with code $code"
    exit 1
fi

chmod +x "$DOWNLOAD_FILE"
```
