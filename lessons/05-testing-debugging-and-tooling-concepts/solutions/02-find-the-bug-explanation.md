# Explanation 02: Find the Bug

The bug exists because the fallback arm returns the existing state for any event
that does not match an earlier branch. That includes `"disconnect"` when the
state is `Connected`.

This is a good lesson in why broad default branches can hide missing cases. The
test is important because it prevents the same regression from returning later.
