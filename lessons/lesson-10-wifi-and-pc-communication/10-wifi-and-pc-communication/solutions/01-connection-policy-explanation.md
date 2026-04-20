# Explanation 01: Connection Policy

This policy is intentionally simple. The goal is not to invent the smartest
reconnect strategy. The goal is to make behavior explicit and observable.

Silent retry loops are a bad default because they hide failure.
