#!/bin/bash

#- Add transactions as specified in Scenario A (Readme)

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "UNILEVER", "points": 200, "timestamp": "2020-10-31T11:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "DANNON", "points": -300, "timestamp": "2020-10-31T15:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "DANNON", "points": 500, "timestamp": "2020-10-31T10:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "MILLER COORS", "points": 500, "timestamp": "2020-10-31T18:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "UNILEVER", "points": 600, "timestamp": "2020-11-12T10:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "STARBUCKS", "points": 200, "timestamp": "2020-11-12T11:30:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "UNILEVER", "points": -200, "timestamp": "2020-11-12T11:30:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "DANNON", "points": 900, "timestamp": "2020-11-12T13:30:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "MILLER COORS", "points": 5000, "timestamp": "2020-11-12T18:20:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "VALVE", "points": 800, "timestamp": "2020-11-18T15:35:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "OLD CHICAGO", "points": 800, "timestamp": "2020-09-28T10:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "OLD CHICAGO", "points": -250, "timestamp": "2020-11-04T10:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "SOUTHWEST", "points": 2500, "timestamp": "2020-11-01T13:50:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "OLD CHICAGO", "points": 300, "timestamp": "2020-11-05T12:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "VALVE", "points": 1200, "timestamp": "2020-11-01T13:50:00Z" }' \
  http://localhost:8000/point-transaction

#- Check both total points and by payer

curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8000/points

printf "\n"

curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8000/points-sum

printf "\n"
