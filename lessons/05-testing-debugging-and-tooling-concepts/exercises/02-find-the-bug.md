# Exercise 02: Find the Bug

## Objective

Practice reading state-machine logic and spotting missing behavior.

## Task

Assume the intended rule is:

- when the current state is `Connected`
- and the event is `"disconnect"`
- the result should become `Disconnected`

Tasks:

- explain why the current function misses this rule
- rewrite the function
- add a test that would fail before the fix and pass after the fix

## Hints

- look carefully at the fallback branch
- missing behavior is often hidden by a default case
- the best bug-fix test describes the missing rule directly

## Why this exercise exists

This is a realistic debugging pattern: the function mostly works, but one branch
is missing and a test is the safest way to lock in the fix.
