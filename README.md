This repo contains the assignment (`ASSIGMENT.md`) and solution to a technical case I recently did at a large anonymous company. There's [a linkedin post](https://www.linkedin.com/posts/martin-zacho_github-mzachoknapsack-service-activity-7218992159180390400-tF31?utm_source=share&utm_medium=member_desktop) (in Danish) accompanying it.

## Services

API:

- Defines routes:
  * GET /knapsack/<id>
    + Reads status and possibly solution of knapsack with <id> from
      a Postgres database.
  * POST /knapsack with knapsack object as body
    + Inserts the knapsack object into the `tasks` table of the db.
    + Publishes messages to RabbitMQ using AMQP 0-9-1 on queue
      `problem_submitted` containing the id of the knapsack object.

Optimizer:

- Sets up a consumer on the `problem_submitted` queue.
- When new messages arrives with some `task_id`, then the service
  reads the problem from the database and updates the entry by setting
  its status to `started` and `ts_started` to the current unix/epoch
  time.
- After solving the problem then the entry in the `db` is again
  updated by setting status to `completed` and `ts_completed` to
  current the unix/epoch time.

## Building and running

Run `docker-compose up` from within this directory to start an API
server and 2 instances of the optimizer as well as Postgres and
RabbitMQ instances.

After the services has started up, then the API is available on port
6543 of the localhost, and new tasks can be created like this:

```bash
$ curl -XPOST -H 'Content-type: application/json' http://localhost:6543/knapsack \
   -d '{"problem": {"capacity": 60, "weights": [10, 20, 33], "values": [10, 3, 30]}}'
{"task":"2881a781-9c3d-4eba-b450-290d98c68026","status":"submitted","timestamps":{"submitted":1720683467,"started":null,"completed":null},"problem":{"capacity":60,"weights":[10,20,33],"values":[10,3,30]},"solution":{}}%
```

And solutions queried like this:

```bash
 curl -XGET -s http://localhost:6543/knapsack/2881a781-9c3d-4eba-b450-290d98c68026 | jq .
{
  "task": "2881a781-9c3d-4eba-b450-290d98c68026",
  "status": "completed",
  "timestamps": {
    "submitted": 1720683467,
    "started": 1720683467,
    "completed": 1720683468
  },
  "problem": {
    "capacity": 60,
    "weights": [
      10,
      20,
      33
    ],
    "values": [
      10,
      3,
      30
    ]
  },
  "solution": {
    "packed_items": [
      0,
      2
    ],
    "total_value": 40
  }
}
```

## Todo

Postgres user management

RabbitMQ user management

services: Use pooled db connections (I imagine having only
a single db connection and guarding it behind a mutex is
currently the bottleneck in terms of response times)

deploy to k8
