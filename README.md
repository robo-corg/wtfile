# wtfile

_This is mostly a prototype to explore what can be done with llms in this space! Use at your own peril._

Looks up information on files similar to that of the unix command file `file` but uses an llm.

Currently the tool only uses the name of the file. The plan is to eventually use the output of `file` and search results as well. Even without this information many files have fairly useful results.

## Setup

_WIP sorry!_

Set the `OPENAPI_KEY` to your anthropic api key. Other backends can be used by configuring the base_url and model.

## Examples

The default `claude-3-5-sonnet-20241022` seems to perform the best (besides claude-3.7).

```
wtfile /etc/magic.mime

The `/etc/magic.mime` file is a configuration file used by the `file` command on Unix/Linux systems specifically for MIME type detection. It contains patterns and rules that help identify file types and their corresponding MIME (Multipurpose Internet Mail Extensions) types.

Key points about /etc/magic.mime:

1. Purpose:
- Used to determine the MIME type of files
- Helps applications and web servers properly handle different types of files
- Part of the file identification system

2. Used by:
- The `file` command with the `-i` or `--mime` options
- Web servers like Apache
- Various applications that need to identify file types

3. Format:
- Contains magic number patterns and corresponding MIME types
- Each entry describes characteristics that identify specific file types
- Follows a structured format for pattern matching

4. Related commands:
- `file -i filename` (shows MIME type of a file)
- `file --mime-type filename`
- `update-mime-database` (updates MIME database)

This file is important for proper file type detection and handling in Unix/Linux systems, especially for web servers and email systems that need to properly identify file types.
```

## Wby I built this

Mostly as an experiment to see what sort of tools can be built with llms that actually might be useful.