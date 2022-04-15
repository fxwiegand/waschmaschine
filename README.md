![Rust](https://github.com/fxwiegand/waschmaschine/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/d/fwaschmaschine.svg?label=crates.io%20downloads)](https://crates.io/crates/waschmaschine)
[![Crates.io](https://img.shields.io/crates/v/waschmaschine.svg)](https://crates.io/crates/waschmaschine)
[![Crates.io](https://img.shields.io/crates/l/waschmaschine.svg)](https://crates.io/crates/waschmaschine)
[![codecov](https://codecov.io/gh/fxwiegand/waschmaschine/branch/main/graph/badge.svg?token=RX8MBLC73T)](https://codecov.io/gh/fxwiegand/waschmaschine)

# waschmaschine
A darts checkout api written in rust

### API Routes
`/checkout/<score>`
 
##### Example:
`/checkout/170`

```json
{
  "darts":[
    {
      "field":20,
      "region":"Triple"
    },
    {
      "field":20,
      "region":"Triple"
    },
    {
      "field":25,
      "region":"Double"
    }
  ]
}
```



