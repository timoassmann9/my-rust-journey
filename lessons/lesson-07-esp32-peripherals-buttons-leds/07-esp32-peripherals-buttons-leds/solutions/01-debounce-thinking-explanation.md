# Explanation 01: Debounce Thinking

The important part is not the exact debounce number. It is the idea that raw
hardware signals should often be filtered before they become application events.

That separation helps you avoid mixing hardware noise handling with business
logic.
