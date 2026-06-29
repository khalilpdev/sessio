# Sessio

**Remember everything. Continue anywhere.**

Sessio is an open-source, cross-platform developer memory platform built in Rust.

It automatically indexes your Git repositories, AI coding assistant sessions, development workspaces, programming languages, commits, and project history into a single searchable local database.

Unlike traditional AI chat history, Sessio reconstructs your entire development context, allowing you to instantly resume work on any project, regardless of which AI tool you used.

## Why?

Modern development is fragmented.

A single project may involve GitHub Copilot CLI, OpenCode, Claude Code, ChatGPT, Git, multiple terminals, several branches, and dozens of unfinished conversations.

Each tool remembers only part of the story.

Sessio remembers everything.

## Features

- 🚀 CLI-first experience
- 🦀 Built with Rust
- 🖥 Cross-platform (Windows, Linux, macOS)
- 🔍 Automatic project discovery
- 🤖 AI session indexing
- 🌿 Git integration
- 📊 Local dashboard
- 🔎 Full-text search
- 📚 Timeline of development activity
- 🔌 Plugin architecture
- 🔒 Privacy-first (local SQLite database)
- ⚡ Fast incremental indexing

## Current status

Sessio now includes an initial Rust CLI bootstrap.

- `cargo run -- --help` shows the command surface
- `sessio index`
- `sessio search <query>`
- `sessio recent`
- `sessio continue`
- `sessio dashboard`

Today these commands provide friendly placeholders while the indexing, search, timeline, and dashboard features are built out on top of the local SQLite foundation.

## Example

```bash
sessio index

sessio search jwt

sessio recent

sessio continue

sessio dashboard
```

## Philosophy

Git remembers your code.

AI remembers conversations.

Sessio remembers your development journey.
