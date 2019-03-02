
+++
date = "2015-12-12"
draft = false
title = """Trusted Client Domains with Ghost's Bleeding Edge Public API"""
slug = "ghost-api-trusted-client-domain"
tags = ['technology']
banner = ""
aliases = ['/ghost-api-trusted-client-domain/']
+++

My first term at university has come to an end and to kill a Saturday on the barely qualifying fringes of Greater London I've set myself the challenge of revamping my landing page in 24 hours. I'm about half way through that time period and I've got a fairly bare bones template that I'm trying to make more interesting by accessing information from the various places I inhabit on the web.

One of the places I'd like to pull content down from is this blog which is running on the lightweight [Ghost](https://ghost.org) platform. Ghost is a relatively new kid on the block and their API has only just really come out and has to be enabled via an obscure tick box in the 'Labs' settings page. In addition if you want to query the API from a different domain to the one your blog is hosted on, like I do you'll need to add a new trusted domain to the `client_trusted_domains` table in the SQLite3 database.
