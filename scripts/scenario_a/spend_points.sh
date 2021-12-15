#!/bin/bash

# Spent points as specified in the exercise PDF

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "points": 400 }' \
  http://localhost:8000/spend-points

printf "\n"

#- Check both total points and by payer

curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8000/points

printf "\n"

curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8000/points-sum

printf "\n"