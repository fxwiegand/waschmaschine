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



