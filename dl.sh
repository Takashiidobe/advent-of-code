#!/usr/bin/env bash

for year in $(seq 2015 2023); do
  url="https://adventofcode.com"
  url_day="${url}/$year/day"
  mkdir -p aoc
  cd aoc
  mkdir -p static
  curl "${url}/static/style.css" > static/style.css
  mkdir -p $year/day
  cd $year/day

  for i in $(seq 25); do
    mkdir -p $i
    curl "${url_day}/${i}" > ${i}/index.html
    curl -H "Cookie: session=$(cat $HOME/.config/adventofcode.session)" \
      -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0' \
      -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8' \
      -H 'Accept-Language: en-US,en;q=0.5' \
      -H 'Accept-Encoding: gzip, deflate, br' \
      -H "Referer: https://adventofcode.com/${year}/day/${i}" \
      -H 'DNT: 1' \
      -H 'Connection: keep-alive' \
      -H 'Upgrade-Insecure-Requests: 1' \
      -H 'Sec-Fetch-Dest: document' \
      -H 'Sec-Fetch-Mode: navigate' \
      -H 'Sec-Fetch-Site: same-origin' \
      -H 'Sec-Fetch-User: ?1' \
      -H 'Pragma: no-cache' \
      -H 'Cache-Control: no-cache' \
      "${url_day}/${i}/input" > ${i}/input
  done

  cd ~/advent-of-code
done
