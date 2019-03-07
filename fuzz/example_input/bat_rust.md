+++
draft = false
slug = "rust-alternatives-to-command-line-utilites-bat-cat"
tags = ['Rust', 'Bash', 'Cat', 'Unix']
banner = ""
title = 'Replacing Cat with Bat'
date = "2019-01-06"
+++

Any frequent user of a Unix operating system will find themselves using the same command line utilities over and over again: rm, mv, ls, cp, grep, and cat to name a handful.

A lot of these utilities are old and are written in C. C is a great language for high performance work but it's long in the tooth and it's speed often comes at the cost of reliability and/or security. Of course, whilst classic Unix utilities like cat are battle tested they are often difficult to change. Decades old codebases aren't the easiest thing to work with and doing so in a way that is backwards compatible and with the same level of assurance of reliability isn't trivial. Thankfully [Rust](https://www.rust-lang.org/) provides a solution - a language with safety and performance at it's heart.

Recently I stumbled across a 'rustified' version of cat - [Bat](https://github.com/sharkdp/bat). Bat is a true cat replacement that also includes syntax highlighting, Git integration, and automatic piping to less - awesome!
