# Anya-scraper 🕊

A scraper for Mahjong Soul stickers in `https://tiermaker.com/create/mahjong-soul-stickers--1267118` using an HTML excerpt from the page.

Inspired by Anya, who wanted some Soul's stickers for the mahjong club's tournament poster. 🀄

![knife](imgs/191.png)

It uses parallelization, with `rayon`'s parallel iterators, to make the url fetching from the `source.html` file with `scraper` and the image downloading with the `reqwest` and `image` crates faster.

Using a single for loop gives a time of 81.988s compared to previous attempts of 90s or more. 🌟

![keyboard](imgs/537.png)
