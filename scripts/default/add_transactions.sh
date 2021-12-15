#!/bin/bash

#- Add transactions as specified in the exercise PDF

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "DANNON", "points": 1000, "timestamp": "2020-11-02T14:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "UNILEVER", "points": 200, "timestamp": "2020-10-31T11:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "DANNON", "points": -200, "timestamp": "2020-10-31T15:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "MILLER COORS", "points": 10000, "timestamp": "2020-11-01T14:00:00Z" }' \
  http://localhost:8000/point-transaction

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "DANNON", "points": 300, "timestamp": "2020-10-31T10:00:00Z" }' \
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
