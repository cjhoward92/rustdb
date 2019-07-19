# Rust DB

A key-value database written completely in Rust.

## Design

Rust DB is a simple key-value database. The overall goal is to 1) learn Rust and 2) learn how to build a simple, relatively reliable database. Eventually, if given enough time, this database might become actually reliable in a _real_ sense. Not just as a toy. I doubt that, but a man can dream... Anyway, this is just a fun project.

The initial implementation will be simple and support a single data type: strings.

Keys will be strings, values will be strings. What you put in that string is on you, but for now it's just a string.

**Key rules:**

* Keys cannot be empty or blank strings
* Keys must be unique
* Keys have a max length of `255` characters
* Keys are ASCII only

**Value rules:**

* Values can be UTF-8 string value
* Values cannot be null, instead they will coerced to an empty string
* Values can have a max length of `65535` characters

These very strict limitations are simply to keep the initial implementation clean and easy to reason about. Nothing more. Will these rules be expanded in the future? Possibly. Who knows.
