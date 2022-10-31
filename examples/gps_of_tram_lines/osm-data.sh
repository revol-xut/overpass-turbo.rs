#! /usr/bin/env bash

curl http://overpass-api.de/api/interpreter -X POST -d@- <<EOF
[out:json][timeout:25];
(
  node["type"="route"]["route"="tram"]({{bbox}});
  way["type"="route"]["route"="tram"]({{bbox}});
  relation["type"="route"]["route"="tram"]({{bbox}});
  node["type"="route"]["route"="subway"]({{bbox}});
  way["type"="route"]["route"="subway"]({{bbox}});
  relation["type"="route"]["route"="subway"]({{bbox}});
);
out body;
>;
out skel qt;
EOF
