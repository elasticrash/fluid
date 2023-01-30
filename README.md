# fluid

## Setup Schema

* install sea-orm-cli if needed `$ cargo install sea-orm-cli`
* run the migration `$ sea-orm-cli migrate -u <DATABASE_URL>`

## Configuration

* fluid `-f [filename]`
* defaults to `config.json`
* if no configuration is provided it defaults to `postgresql://local:password@localhost:5432/scheduler`
  
## Endpoints

* POST `/schedule`

## Parameters

* Expression

format `{number}:{period}`

### periods
```text
Y = year
M = month
D = day

h = hour (24 hour format)
m = minute
s = second
```
### example

* `5:m`   =  5 minutes
* `15:D` = 15 days

