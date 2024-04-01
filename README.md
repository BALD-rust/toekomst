# toekomst

User interface library for low-power embedded devices driven by futures.
Assumes a binary coloured display without touchscreen: interaction is done by 'accelerators', triggered by keyboard presses.
Designed as part of a bachelor's thesis at the KU Leuven.

Some of toekomst's primitives are decently documented, and examples can be found in the `examples` subdirectory.

However, please keep in mind that this is a proof-of-concept and is not made for real-world use. toekomst has a hard dependency on [embassy](https://github.com/embassy-rs/embassy), but can theoretically be freed from this if someone extracts the synchronization primitives: PRs welcome.
