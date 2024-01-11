# Research report - Enhancing system performance

Author: Koen Schellingerhout

Date: 2024-01-14

<br>

**Versioning**

| Version | Date       | Revisions                                          | Status |
| ------- | ---------- | -------------------------------------------------- | ------ |
| 0.1.0   | 2023-10-27 | Initial research report                            | draft  |
| 0.2.0   | 2024-01-12 | Change research questions, add results and sources | draft  |
| 0.3.0   | 2024-01-14 |                                                    | draft  |

<div style="page-break-after: always;"></div>

## Introduction

Spaced is a knowledge base application, with an interactive 2D plane to place documents, notes, images, and other "items" next to or nested within each other. Users can create and then share items with others for live-collaboration. Spaced is supposed to be open-sourced and made specifically for self-hosting.

## Problem statement

The responsiveness of Spaced is very important to make it an actual good productivity tool and so updates need to feel fast. Furthermore low costs are important for self-hosting, therefore the system performance is important. The non-functionals as defined in the requirements document require lots of data throughput and with 100.000 concurrent users this will most likely be a challenge.

## Goal of the research

The goal of this research should provide insights on how to ensure the system performance based on the non-functionals are met while dealing with the 100.000 concurrent users target. Spaced is meant to be self-hosted and therefore keeping costs as low as possible is important as well.

## Scope

The scope of the research is limited to finding if the current architecture for Spaced can reach the item querying rates required to hit the target performance, based on the criteria defined in the requirements document.

<div style="page-break-after: always;"></div>

## Research questions

### Main question

How do the existing design decisions for Spaced perform when scaled up to accommodate 100.000 concurrent users working on their items collaboratively in a 2D environment, ensuring sub-second latency between user interactions, and data resiliency with no loss longer than a minute?

### Sub-questions

**Sub-question 1: How can the performance be measured of the system to find bottlenecks within the architectural choices?**

- How can the tests be applied to identify and address potential bottlenecks within the design of Spaced?
- What tests should be conducted to evaluate the relevant system performance attributes?

<ins>Strategies<ins>

<img src="assets/strategy_icons/library.png" width=30 /> ->
<img src="assets/strategy_icons/workshop.png" width=30 /> ->
<img src="assets/strategy_icons/showroom.png" width=30 />

<ins>Methods and justification<ins>

- Scavenging the internet for [good and bad practices](https://ictresearchmethods.nl/Best_good_and_bad_practices) when it comes to measuring system performance will be crucial to figure out how- and what to test within the system.
- [Prototyping](https://ictresearchmethods.nl/Prototyping) just enough to test the most relevant system requirements for the "benchmark tests".
- [Benchmark test](https://ictresearchmethods.nl/showroom/benchmark-test/) the prototypes locally to gain experience and some initial insights to prepare more insightful tests for answering sub-question 2.

<ins>Expected products<ins>

- A list about load- and stress testing best practices.
- Benchmark test results with explanations on what they could tell about the system.
- A list of tests to conduct used for the second sub-question.

**Sub-question 2: How does the system behave under heavy loads?**

- In what ways does the system run into bottlenecks and how does it utilize its resources?
- At what system load does the system start to encounter data loss?

<ins>Strategies<ins>

<img src="assets/strategy_icons/workshop.png" width=30 /> ->
<img src="assets/strategy_icons/lab.png" width=30 /> ->
<img src="assets/strategy_icons/showroom.png" width=30 />

<ins>Methods and justification<ins>

- [Prototype](https://ictresearchmethods.nl/Prototyping) the infrastructure required to perform proper load testing.
- Use [A/B-](https://ictresearchmethods.nl/lab/a-b-testing) and [non-functional testing](https://ictresearchmethods.nl/lab/non-functional-test) to compare results and see how the system performs regarding the non-functional requirements.
- Use [benchmark tests](https://ictresearchmethods.nl/showroom/benchmark-test/) to [product review](https://ictresearchmethods.nl/showroom/product-review/) wether the non-functional requirements are met or not.

<ins>Expected products<ins>

- Various branches in GitHub with different implementations of the system.
- A list of load- and stress test results for each system implementation.

**Sub-question 3: How can Spaced be improved to reach and potentially exceed the target performance?**

Considering the non-functional requirements.

- What specific database choices and configurations can be implemented to improve the system performance?
- What service level improvements can be applied?

<ins>Strategies<ins>

<img src="assets/strategy_icons/library.png" width=30 /> ->
<img src="assets/strategy_icons/workshop.png" width=30 /> ->
<img src="assets/strategy_icons/lab.png" width=30 /> ->
<img src="assets/strategy_icons/showroom.png" width=30 />

<ins>Methods and justification<ins>

- By doing a [gap analysis](https://ictresearchmethods.nl/workshop/gap-analysis) on the parts considered a bottleneck, further refinements can be searched for and applied.
- Additionally do a [literature study](https://ictresearchmethods.nl/Literature_study) to gain insights on other potential solutions.
<!-- - [Prototype](https://ictresearchmethods.nl/Prototyping) the new found solutions and compare them using [A/B testing](https://ictresearchmethods.nl/lab/a-b-testing).
- The systems components will be [benchmark tested](https://ictresearchmethods.nl/Benchmark_test) ensuring the enhancements were successfully applied. -->

<ins>Expected products<ins>

- A gap analysis going through all bottlenecks found.
- A list of suggestions to improve system performance.

<div style="page-break-after: always;"></div>

## Results

### Sub-question 1: How can the performance be measured of the system to find bottlenecks within the architectural choices?

Spaced uses Socket.IO built on top of the websocket protocol to allow for live-collaboration. Finding tooling that works well with Socket.IO is crucial as all item updates are sent to the client using Socket.IO. Artillery a load- and stress testing tool is mentioned by the [1][^1].

[^1]: https://www.artillery.io/docs/get-started/core-concepts "test"

**Load & stress testing concepts**

The tooling mentioned has their own concept when it comes to load- and stress testing.

Artillery.io

- Virtual users
- Phases

Grafana: k6

- Open and closed models | Different ways k6 can schedule VUs, their affects on test results, and how k6 implements the open model in its arrival-rate executors
- Graceful Stop | A configurable period for iterations to finish or ramp down after the test reaches its scheduled duration
- Arrival-rate VU | allocation How k6 allocates VUs in arrival-rate executors
- Dropped iterations | Possible reasons k6 might drop a scheduled iteration

**Good and bad practices**

1. Use a staging or development environment for load tests rather than a production environment. There are some tools that like Speedscale, that allow replaying of production traffic in other environments.
2. Look at the scope of the load tests to perform and use mock APIs to keep the test within scope. Mocking can help avoid unnecessary resource usage which could mitigate higher costs.
3. Use realistic data, look at user behavior, where possible.
4. Determine what metrics to collect and how to collect them, to assess whether performance targets are met.
5. Be cautious about using agents in testing, since it might result in unrealistic behavior. (what is an AGENT?) An agent can impact the application performance. Use "cost-free" metrics collection.

**Initial findings**

Measuring throughput locally of a single item_producer.

1 item vs 10 items vs 100 items

_Expirements can found at ..._

**Scenario's to test**

_Non-functional requirements_

- Performance​ & Scalability​

  - The system must support at least 100.000 concurrent users
  - The system must be able to support at least 20 users sharing live updates per item/space
  - Latency for item updates (including: creating, updating and deleting text and other item metadata) should be visible to the user within 1 second even with 100.000 concurrent users.
  - Shared updates should be visible to other within 5 seconds +/- the latency caused by physical distance.
  - The client must be able to load in at minimum 20 fully loaded items, including 20 1 MB images, and 2x the screen space of text on scaling set to 1.
  - The client must be able to load in at least the metadata of 100 items at a time, which is includes location, size and name.
  - The static content of the app should be small enough to load within 1 second

- Resiliency

  - Loosing user data must not occur for longer than a minute.
  - The uptime of Spaced must not deviate more than 10% of the cloud providers gaurentees.

1. single producer/consumer vs multi producer/consumer
2. single vs multi database setup -> probably not relevant
3. blobs in database vs object storage
<!-- 4. with and without encryption and searchable encryption methods -->
4. Cloud provider vs local system
5. "no consumers"

### Sub-question 2: How does the system behave under heavy loads?

The system currently does not reflect the architecture and so that needs to be built out first.

Cloud provider specs

1. single producer/consumer vs multi producer/consumer
2. single vs multi database setup -> probably not relevant
3. blobs in database vs object storage
<!-- 4. with and without encryption and searchable encryption methods -->
4. Cloud provider vs local system
5. "no consumers"

### Sub-question 3: How can Spaced be improved to reach and potentially exceed the target performance?

The performance ...

<div style="page-break-after: always;"></div>

## Conclusion

<div style="page-break-after: always;"></div>

## Sources

Concepts
https://www.artillery.io/docs/get-started/core-concepts
https://grafana.com/docs/k6/latest/using-k6/scenarios/concepts/

Best practices
https://speedscale.com/blog/what-is-load-testing/
https://www.artillery.io/docs/get-started/best-practices
