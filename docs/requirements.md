Non-functional requirements

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

- Privacy & Security

  - Strong container hardening to ensure minimal impact from supply chain attacks
  - Strong control over data access
  - Rate limiting out of the box
  - Strong encryption for all confidential user data
  - Privacy (GDPR)​
  - item data must be encrypted in the database (this is optional depending on the users' settings or item settings)
  - item data must use true end-to-end encryption when sent to the client
  - username, email, password must be encrypted at all times
