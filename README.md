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
* [Async](https://doc.rust-lang.org/std/keyword.async.html)
* [JSON (de)serialization](https://github.com/serde-rs/json)
* [reading .env variables](https://github.com/dotenv-rs/dotenv)
* [Integrating CLI commands](https://github.com/clap-rs/clap)
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

