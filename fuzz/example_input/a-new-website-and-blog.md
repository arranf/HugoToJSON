+++
draft = false
title = 'A New Website and Blog Theme'
date = "2019-01-06T17:29:52Z"
slug = "a-new-website-and-blog-theme"
tags = ['VuePress', 'Vue', 'Website', 'Blog', 'Hugo', 'CV', 'HemingPress']
+++

My website was overdue a facelift, it was [bland, outdated, look bad on mobile, and missed some key information](https://web.archive.org/web/20180422195203/https://arranfrance.com/). Not only that, it was an obscure Jekyll/HTML mashup with a horrible Gulp/Travis build process - not at all friendly to maintain.

I decided that to add [my CV](https://arranfrance.com/cv) to the website it was time for a rewrite. I considered writing the whole website in regular plain HTML and CSS but I wanted some things to 'just work' like routing and a basic theme but the power to go 'under the hood' when needed to define a unique layout for the CV. I ended up settling on [VuePress](https://vuepress.vuejs.org/). VuePress comes with a great out of the box theme as well as the ability to write Vue components in markdown. For the CV page I defined a custom layout which contains multiple reusable components.

I also adjusted this Hugo blog to match the same theme by converting the old [Hemingway theme](https://github.com/arranf/hemingway) I was using to use VuePress's default styling. I've published that theme [here](https://github.com/arranf/HemingPress). The old theme had a lot of dependencies which have been either been removed or upgraded and I also fixed the build process along the way. Overall, the blog looks a lot cleaner and I feel a lot more confident in its appearance and the structure of its layout.