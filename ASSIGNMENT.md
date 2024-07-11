## Introduction

**Welcome Dear Potential Colleague!**

This case is meant to inspire you to prepare and present a solution to a made-up challenge. The only purpose is to serve as background for an interesting discussion, where you have the chance to display your skills in concrete problem solving and where we have the opportunity to get a glimpse of how you work. This case is not a representation of an existing product, team or stakeholder landscape. Situations and circumstances have been invented to draw up interesting dilemmas and challenges for you to explore.

**Are You Ready?**

The assignment creates a base of discussion for the interview. We do not expect you to use more than a day on the task. It is ok to stub parts of the application in order to focus on other aspects that you think show of your skillset better.

## The optimisation problem

Here at XXXX we work with a variety of real-life optimisation problems. The motivation behind this challenge is to let you experience the process of providing an optimiser service, a task you might expect at work if you join us (though solving a simplified optimisation problem).

One our biggest markets is Albanese country, and specifically ABC province. Due to increasing crime rates our company have, under the advice of its own insurance company, hired an extern security company. The security staff must accompany all deliveries in Albanese. This creates a bottleneck as there is a finite amount of security personal, as well as each delivery (parcel) having a maximal weight it may hold.

Your task is to help our customers plan which of their products to pack into a single parcel delivery (ignoring volume constraints). Because the number of security staff that can accompany our parcels is limited our customers want to ship the most valuable products in their deliveries first.

In computer science, this problem is known as the knapsack problem. One specifies the weights and values of a collection of items and the capacity of a knapsack, and is looking for a way to pack some of the items, up to the capacity constraint, to maximise the total value of the knapsack content. If you're not familiar with the Knapsack problem, consider starting with this relevant [Youtube video](https://www.youtube.com/watch?v=xOlhR_2QCXY) or if you are more of a visual person this relevant [Wikipedia article](https://en.wikipedia.org/wiki/Knapsack_problem).

## The task

You are asked to develop a solution for solving knapsack problems of potentially very large sizes. In particular, it should allow a user to:

* provide a knapsack problem (knapsack capacity + weights and values of items);
* execute the optimiser for the problem;
* retrieve the solution.

For the optimiser you can reuse an existing knapsack problem solver (e.g. [Google Optimization Tools](https://developers.google.com/optimization/bin/knapsack) or another 3rd party provider). If you prefer, you may also implement your own (possibly naive) knapsack solver.

### General guidelines

* Please prepare a short walkthrough of your provided solution for the 3rd interview. Also share the delivery with us beforehand so we can ask more relevant questions.
* Your delivery should be containerized and have no system dependencies besides docker, docker-compose or Kubernetes. If you are not familiar with docker, check out the official Docker Get Started Guide: [Orientation](https://docs.docker.com/get-started/) and [Containers](https://docs.docker.com/get-started/part2/).
* The delivery should include a README.md file with concise and complete instructions on how to use (build, execute and access) your service. Additional documentation (discussions on architecture, code structure and technology stack choices) should also be included.
* Please make your solution accessible only to you and us.



## Delivery specification

Your delivery should:

* We would like to see your thoughts and planned architecture in a diagram (can be on the back of a napkin)
* If you take any shortcuts, please document your choices
* Include instructions on start, use and how to test the solution
* Your solution must be containerized (i.e. it should be executable through Docker. If using multiple containers, you can orchestrate them manually or via docker-compose or Kubernetes)
* Provide a REST API (see below) with endpoints to submit optimization tasks asynchronously (especially important for larger problems that may take substantial time to compute), retrieving the status of tasks and solution of completed tasks.

We would like you to create your solution in the language and environment you are most comfortable with.

### REST API

The user must be able to interact with the solution via the REST API specified here.
See the examples section below for their usage with `curl`.

* POST `/knapsack`
content: `application/json` with JSON knapsack problem specification
output: `json` with JSON knapsack object

* GET `/knapsack/<id>`
output: `json` with JSON knapsack object

#### JSON specifications

The `json` requests and responses should have the following formats:

```json
# problem specification
{
    "problem": {
        "capacity": # non-negative integer
        "weights": # array of non-negative integers
        "values": # array of non-negative integers, as many as weights
    }
}

# knapsack object
{
    "task": # Task ID (ASCII string)
    "status": # one of "submitted", "started", "completed"
    "timestamps": {
        "submitted": # unix/epoch time
        "started": # unix/epoch time or null if not started
        "completed": # unix/epoch time or null if not completed
    }
    "problem": # problem specification as above (including capacity, weights, values)
    "solution": {  # if completed
        "packed_items" : # array of integers (indices to weights and values)
        "total_value": # sum of value of packed_items
    }
}
```

Please make sure your APIs follow these specifications, as we will be testing your solution with an automated suite of problems and solutions.

### Example session

The following is a hypothetical session with a knapsack optimiser service, using for example [Tilt](https://docs.tilt.dev/tutorial/index.html) or docker-compose to start a composite service, listening on port 6543:

```bash
$ cd <root directory of the service>
$ tilt up
...
$ curl -XPOST -H 'Content-type: application/json' http://localhost:6543/knapsack \
   -d '{"problem": {"capacity": 60, "weights": [10, 20, 33], "values": [10, 3, 30]}}'
{"task": "nbd43jhb", "status": "submitted", "timestamps": {"submitted": 1505225308, "started": null, "completed": null}, "problem": {"capacity": 60, "weights": [10, 20, 33], "values": [10, 3, 30]}, "solution": {}}

{"task":"9c8d3a92-","status":"submitted","timestamps":    {"submitted":"1720534132","started":null,"completed":null},"problem":{"capacity":60,"weights":[10,20,33],"values":[10,3,30]},"solution":{}}%

$ curl -XGET -H http://localhost:6543/knapsack/nbd43jhb
{"task": "nbd43jhb", "status": "started", "timestamps": {"submitted": 1505225308, "started": 1505225342, "completed": null}, "problem": {"capacity": 60, "weights": [10, 20, 33], "values": [10, 3, 30]}, "solution": {}}

$ curl -XGET -H http://localhost:6543/knapsack/nbd43jhb
{"task": "nbd43jhb", "status": "completed", "timestamps": {"submitted": 1505225308, "started": 1505225342, "completed": 1505225398}, "problem": {"capacity": 60, "weights": [10, 20, 33], "values": [10, 3, 30]}, "solution": {"packed_items": [0, 2], "total_value": 40}

$ _
```
