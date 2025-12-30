#### Good-To-Knows
IDE: RustRover by JetBrains
Language: Rust
Framework: Axum
Operating System (OS): Ubuntu 24.0
Architecture: x86_64

# Prompting
Nothing fancy, minimal text is required.

If you must add text, make it clear and concise.
There is no need for any type of grammar, keep it to the basics (periods and commas).

# Code Styling

You must make variables immutable if they can be.
Use guard-clauses to prevent sluggish code.
Code should explain itself, do not add comments unless it's a really, really weird block of code.
Never add documentation unless said otherwise in the prompt.
Accept all Rust's code styling.

# Critical Thinking

Before changing any line of code, ensure it's the proper way of fixing the bug/syntax error.
The smaller, the better. If an error (or bug) can be fixed easily, use that method instead.'

When anybody asks you to fix anything, run the codebase first to get errors, debug output, or traces.

After you run the code, find clues in the output, then carefully change the code. 
Run the codebase after small changes and if something breaks, just simply revert it and find a new way of doing things

# Thinking Before Executing

Always think before executing, if a change is redundant and doesn't actually fix the problem, don't do it!
Changing anything that I don't ask you to change is not acceptable.

Do not go above-and-beyond if I don't say so, this will cause more problems.

# Context

It's recommended to look through the project even if the file you are editing isn't relevant.

When you're editing something that requires refactoring, keep it all one code style.
Refactoring should never change an entire file.