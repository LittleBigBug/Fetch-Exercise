## Fetch Programming Exercise

This was written in Rust using [Rocket.rs](https://rocket.rs)

It just stores everything in memory using lazy_static with thread safety (Arc, RWLock), no database was used.

This was created as an interview exercise/test for Fetch Rewards. 
It demonstrates my understanding with HTTP requests, looking ahead for edge-cases, working with git, and writing clean, maintainable code.

You can download binaries in [Releases](https://github.com/LittleBigBug/Fetch-Exercise/releases/tag/v1.0.0), or you can compile yourself:

### Compiling

- Install [RustUp](https://rustup.rs/)
- Clone this repo, CD into it
- Opt for Rust Nightly for this project: `rustup override set nightly`
- And then build: `cargo build --release` or debug with `cargo build` (Provides some more logging)
- Binaries will be found in `./target/release/`!

## Using

You can use an HTTP client like Postman, Insomnia, or cURL:

### Adding transactions

**(Complete script in ./scripts/default)**
```shell
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "DANNON", "points": 1000, "timestamp": "2020-11-02T14:00:00Z" }' \
  http://localhost:8000/point-transaction
  
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "payer": "UNILEVER", "points": 200, "timestamp": "2020-10-31T11:00:00Z" } ' \
  http://localhost:8000/point-transaction
  
# Etc
```

### Checking points

```shell
# Will return something like:
#
# {"MILLER COORS":10000,"DANNON":1100,"UNILEVER":200}

curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8000/points

# Will return something like:
#
# {"total-points":11300}

curl --header "Content-Type: application/json" \
  --request GET \
  http://localhost:8000/points-sum
```

### Spending points

```shell
# Initially I made this way first because I thought it was easier,
# but decided to still do the other way because it was how the PDF specified.

curl --header "Content-Type: application/json" \
  --request POST \
  http://localhost:8000/spend-points/5000
  
### OR

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{ "points": 5000 }' \
  http://localhost:8000/spend-points

```

## Issues I had

Accounting for negatives later on in the loop. The given scenario isn't that hard to deal with, but other edge cases will get in the way.

### Scenario A

- DANNON +500
- UNILEVER +200
- DANNON -300

**Total Points:** 400

Let's say, you want to *spend* exactly those 400 points.

In the default loop accounting for the negative isn't that hard.

However, in Scenario A, lets say the user wants to spend 400 points exactly.


In the first loop, it will take only 400 points from the first (What it needs from Dannon+500) and then ignore the second set of points since it already met its goal. However, in the next it sees that it needs to subtract 300, so it sits at 100 in the goal.



