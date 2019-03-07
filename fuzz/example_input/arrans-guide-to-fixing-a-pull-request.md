
+++
date = "2015-07-02"
draft = false
title = """Arran's Guide to Fixing a Pull Request"""
slug = "arrans-guide-to-fixing-a-pull-request"
tags = ['tutorial', 'git', 'technology']
banner = ""
aliases = ['/arrans-guide-to-fixing-a-pull-request/']
+++

So today I was feeling proud as I fixed a bug with a single line change. I changed `if (_groupId != -1)` to `if (_groupId < 0)`. I summed up my change in a simple commit message, created a pull request to share my change and continued making the necessary changes to the remote server.

When I tested the changes on the remote server I had a panic moment when I realised my simple change didn't work. I had got my inequality the wrong way around! The change I'd made was checking if _groupId was **less** than 0 rather than **greater** than 0. Oops. 
