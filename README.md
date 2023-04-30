# REST API web application for derivative pricing and market data storage

# Architecture 

# Tech Stack

## [Rust](https://www.rust-lang.org/) for web sevrer and compute
Although we could easily achieve the same output with Python and Django/Flask the **performance sensitive** nature of our task begs for a better solution. I chose Rust, a modern language compiled to Assembly just like C, for the reasons outlined below:
- No garbage collector, making our server extremely [fast](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)
- **async** frameworks like [Actix](https://github.com/actix/actix-web)
- **multi-core utilization** for compute intensive work (options pricing). Unlike Python, we are not limited by GIL and hence can utilise all cores of the running host. 
- Easy integration with Python. For example [ultibi](https://ultimabi.uk/ultibi-frtb-book/) and [polars](https://github.com/pola-rs/polars) are written in Rust and have very well-functioning python interface (for more details see [pyo3](https://github.com/PyO3/pyo3))

## MongoDB
Naturally, Market Data comes in various shapes and forms. A spot is a totally different object then say Volatility Surface. Hence, Non Structured database would be preffered to 
- Importantly we use **name** (*name of the market data object, eg US_OIS*) and **as_of** (*observation date of the object*) as a unique index. 

# How to Run
## Prerequisites 
 - docker
