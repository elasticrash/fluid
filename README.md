# fluid 🌊

This is just a POC of a db driven scheduling service using RUST. There are many use cases that are not covered or are not desireable. Use it at your own risk!! 
Here is a list I could think of:
i.e
* if the service stops -- it just fires all past missed events (not desirable in some cases, should be configurable)
* start an event in the future -- there but not tested
* deleting an event -- events can't be deleted yet, which means event without an end date will keep going forever, there is a need for an overall event management
* accuracy is 1 second + network latency

## Setup Schema

* install sea-orm-cli if needed `$ cargo install sea-orm-cli`
* run the migration `$ sea-orm-cli migrate -u <DATABASE_URL>`
* adding the option `refresh` on the above command drops and reapplies the schema

## Configuration

* fluid `-f [filename]`
* defaults to `config.json`
* if no configuration is provided it defaults to `postgresql://local:password@localhost:5432/scheduler`
  
## Endpoints

* POST `/schedule` (schedule an event)

``` 
{
    "expression": "30:s",
    "endpoint": "http://127.0.0.1:8000/loop"
    "finish": "2023-3-13T14:12:10", -- optional
}
```

* GET `/loop` (test an event)

## Parameters

### expression

format `{number}:{period}`

#### periods

```text
Y = year
M = month
D = day

h = hour (24 hour format)
m = minute
s = second
```

#### example

* `5:m`   =  5 minutes
* `15:D` = 15 days

### endpoint

Call back URL query string parameters (GET)
* name = name of the task
* time = time of the task

## Structure
* db : db entities (automatically generated)
* common: common code between all modules
* fluid: api
* generator: reads tasks from the db and create events
* processor: reads events from the db and fires webhook events
* migrate: sea-orm db schema
