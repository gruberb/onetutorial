# OneTutorial - A practical journey into the Rust Programming Language

This little project will help you touch many topics around Rust, in a small and contained way. It will touch

* [struct](https://doc.rust-lang.org/std/keyword.struct.html)
* [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
* [enum](https://doc.rust-lang.org/std/keyword.enum.html)
* [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
* [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
* [impl block](https://doc.rust-lang.org/std/keyword.impl.html)
* [traits](https://doc.rust-lang.org/std/keyword.trait.html)
* [Error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
* [Result](https://doc.rust-lang.org/std/result/)
* [Async](https://doc.rust-lang.org/std/keyword.async.html)
* [Handling datetimes](https://github.com/chronotope/chrono)
* [Static values](https://github.com/rust-lang-nursery/lazy-static.rs)
* [JSON (de)serialization](https://github.com/serde-rs/json)
* [reading .env variables](https://github.com/dotenv-rs/dotenv)
* [Integrating CLI commands](https://github.com/clap-rs/clap)
* [Logging](https://github.com/estk/log4rs)
* [Reading/writing from/to a file](https://doc.rust-lang.org/std/fs/struct.File.html)
* [Cross-compiling Rust code](https://blog.rust-lang.org/2016/05/13/rustup.html) 
* [Sending E-Mails via Rust](https://github.com/vokeio/rust-sendmail)
* [Modularising your code](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch07-01-mod-and-the-filesystem.html)
* [Testing](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) 
* [Macros](https://doc.rust-lang.org/reference/procedural-macros.html)


## What you will build

You can adjust the tutorial to your needs, but I find it more usefull to create something you will actually use. Therefore: 

> Update and read from a Google Sheet where you store your latest financial information and send out an E-Mail with a summary. 

This can be adjuste to:

> Update and read from a **CSV file** where you store your latest financial information and send out an E-Mail with a summary.

If you are not interested in the topic (finance), feel free to adjust it to whatever data inputs you consume a lot and want to automise. 

## Before you begin

If you want to go down the Google Sheets route, you need to:

1. [Create a "Service Account"](https://cloud.google.com/iam/docs/creating-managing-service-accounts) via the [Google Console](https://console.cloud.google.com/).
2. [Create a key](https://cloud.google.com/iam/docs/creating-managing-service-account-keys) which downloads a JSON file you need for the library we are going to use.
3. Create a new Google Sheet and share it with the Service Account E-Mail so the secret you just downloaded can read and modify the Google Sheet.

Depending on which external APIs you are going to use, you need to create API keys for them as well. For this tutorial, I am using:

1. The [coinmarketcap.com API](https://coinmarketcap.com/api/documentation/v1/) to fetch the latest Crypto prices
2. The [eodhistoricaldata.com API](https://eodhistoricaldata.com/) to fetch ETF prices 

# The Tutorial 

1. Send a GET request to https://httpbin.org/ip and print the result to the console.
2. Send out a request to CMC and fetch the price of BTC. 
3. Read the API key from an .env file.
4. Get the name, symbol, price and 7day of the BTC and store it in a struct.
5. Pass the list of currencies to fetch via the CLI. 
6. Save the result in a CSV file.
7. In case the API returns an error, write it out to a log file and abort the application.
8. In addition to the coin prices, fetch the price of a random ETF and store it also in a struct.
9. Add a rows in your Google Sheet with ISN, amount of coins, price, total and a row with the total value of your portfolio.
10. Instead of saving the result to a CSV, update your Google sheet.
11. Move out your business logic in different modules.
12. Build your Rust code and move the binary to a server and run it from there.
13. Send out an E-Mail with the coin and ETF overview and redeploy your application/binary.

## Solutions

### Step 1 - Send HTTP request

[Solution in branch step_1](https://git.sr.ht/~gruberb/onetutorial/commit/step_1)


### Step 2 - Send paramterised HTTP GET to CMC

[Solution in branch step_2](https://git.sr.ht/~gruberb/onetutorial/commit/step_2)

### Step 3 - Read API key from .env instead of hardcoded

[Solution in branch step_3](https://git.sr.ht/~gruberb/onetutorial/commit/step_3)

### Step 4 - Store result in custom struct 

[Solution in branch step_4](https://git.sr.ht/~gruberb/onetutorial/commit/step_4)

### Step 5 - Pass list of currencies via CLI 

[Solution in branch step_5](https://git.sr.ht/~gruberb/onetutorial/commit/step_5)

### Step 6 - Save results in CSV file

[Solution in branch step_6](https://git.sr.ht/~gruberb/onetutorial/commit/step_6)

### Step 7 - Logging

[Solution in branch step_7](https://git.sr.ht/~gruberb/onetutorial/commit/e5fcab808e3864acb6f6f6a5abaa615ecd2a5441)

### Step 8 - Fetch ETF from different API

[Solution in branch step_8](https://git.sr.ht/~gruberb/onetutorial/commit/step_8)

### Step 9 - Prepare Google Sheets

This step is done in the browser. 

### Step 10 - Adding GoogleSheets library

[Solution in branch step_10](https://git.sr.ht/~gruberb/onetutorial/commit/step_10)

### Step 11 - Move logic in modules

[Solution in branch step_11](https://git.sr.ht/~gruberb/onetutorial/commit/step_11)

### Step 12 - Cross-compile your code via musl

This is done mostly locally. 

```
> rustup target add x86_64-unknown-linux-musl
> cargo build --release --target=x86_64-unknown-linux-musl
```

You need to add a ssl dependency to `Cargo.toml`, and depending on your OS, install other third party packages. The error message is quite helpful.

[Solution in branch step_12](https://git.sr.ht/~gruberb/onetutorial/commit/step_12)
