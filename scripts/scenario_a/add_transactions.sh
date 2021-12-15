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

# This transaction can be omitted to test that it properly uses the oldest given points.

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "MILLER COORS", "points": 500, "timestamp": "2020-10-31T18:00:00Z" }' \
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
