## Log

### next

- refactor
- gate current functionality behind `etch lists regen`

### 2024-05-08

- successfully converted lists into markdown + wrote to list folder

- successfully drawing lists from frontmatter
- regenerate lists vs changing lists?
  - regeneration is easier but should figure out how to just add links vs whole files

- successfully getting raw filenames out -- ideal for crosslinking
  - it's a little janky though: there are some intermediate variables

- using (dirs)[^5] for config folder management
  - attempt to solve error from `read_to_string`:
  ```
  Running `target/debug/etch`
  2024-05-08T23:17:38.692Z DEBUG [etch] attempting to read config from ~/.config/etch/config.toml...
  Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
  ```

- generalized backlog manager to list manager

- begun work on suite of tools for personal knowledge management
- basic features noted (markdown compat, file-based configuration)

- with specific tool: [[#List manager]]

## Features

### Interface

- command line interface using (clap)[^1]
- full-word "natural-language" interface inspired by cargo
  - e.g. `etch lists update`
- intended to be aliased in shell of choice

### Markdown compatible

- generally compatible with Markdown per (commonmark)[^2]

### Configuration

- reads configuration file per run to determine more permanent settings

### Web integration (TODO)

- no idea yet
- firefox extension?
  - insert links
  - open link from category
- category handling? open `name` in different website based on `category`

#### Variables

- location of list file `lists_dir`
- search directory `search_dir`

## Commands

### Template manager (TODO)

- `etch template`

### List manager

#### Commands

- `etch lists regen`
  - searches `search_dir` for backlog items, using:
    - frontmatter, reading `list`, `name`, `category`
  - If `($NAME)[$FILEPATH]` isn't under the H2 `category` in the file named `$LIST.md` in `lists_dir`, add it

#### Config

- `[lists]` (REQ)
  - `search_dir`: (REQ) search for new backlog items in markdown files in this directory (recursive)
  - `lists_dir`: (REQ) path to the directory containing list files (must be in search dir)

- `[lists.special]`
  - `$LIST`: lists named this get special treatment

## Notes

- built for personal use with (Helix)[^3] and (Marksman)[^4]

[1]: https://crates.io/crates/clap
[2]: https://commonmark.org/
[3]: https://github.com/helix-editor/helix
[4]: https://github.com/artempyanykh/marksman
[5]: https://docs.rs/dirs/latest/dirs/index.html
