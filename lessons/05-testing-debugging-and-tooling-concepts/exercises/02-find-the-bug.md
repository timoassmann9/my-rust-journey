# Exercise 02: Find the Bug

Assume the intended rule is:

- when the current state is `Connected`
- and the event is `"disconnect"`
- the result should become `Disconnected`

That behavior is currently missing.

Tasks:

- identify why the current function fails that requirement
- rewrite `next_state` to include the missing transition
- write a test that would catch the bug

What this trains:

- reading state-machine code
- spotting missing branches
- using tests to lock behavior
