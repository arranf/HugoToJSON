+++
draft = false
title = "Building a Rust Utility: Hugo Static Blog to Lunr Index"
date = "2019-01-13T23:29:54Z"
slug = "building-rust-utility-hugo-to-lunr-json"
tags = ['Hugo', 'Blog', 'Rust', 'Lunr']
banner = ""
+++

Rust is a language which has fascinated me since I was first introduced to it at the [end of my first year of undergraduate](https://blog.arranfrance.com/post/summer-2016/). Since then Rust has evolved at a frantic pace and I feel like I've forgotten a lot of the subtleties of Rust syntax so I've decided to do a number of small projects in Rust to brush up.

The first project I've tackled is replacing an outdated an abandoned npm package, [hugo-lunr](https://www.npmjs.com/package/hugo-lunr). This blog's search is powered by Lunr which requires an array of JS objects containing information about each post. `hugo-lunr` is designed to run during the site's build step to produce the static index by iterating over Hugo markdown files and extracting key information from the front matter and dumping the contents to a JSON file which can be retrieved and consumed by Lunr at runtime. Unfortunately `hugo-lunr` is a fairly bare-bones implementation with a number of open pull requests and issues making it an excellent candidate for replacement.

# Version 0.1
