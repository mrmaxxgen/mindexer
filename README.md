# mindexer
a simple file indexer written in Rust

mrmaxxgen (c) 2026, licensed under the MIT license <https://mit-license.org>

mindexer is a file indexer/sorter utility for automatic by-type file sorting/indexing in specific folder, it can be used for servers html's pages or just for tidying up your system with zero effort, mindexer handles (almost) everything by itself thru fs/env calls and error handling, it will also help you creating a base to start using it.

current version: 0.1.1 BETA

for future releases:
    -add support for other wildcards
    -add reverse index feature

changelog:

0.0.1 ALPHA (pre-release):
    -created repo and code base
    -coded main code, not working, needs fix
    -implemented mp3 and mp4 support for now

0.0.2 ALPHA (pre-release):
    -ditched commands and used rust fs/env libraries instead
    -commented code more
    -made code lighter

0.1.0 BETA:
    -fixed wildcard issue
    -deleted useless code parts
    -made code stable
    -fixed renaming issues
    -added sortfolder cleaning feature
    -configured makefile

0.1.1 BETA:
    -added support for more wildcards
    -deleted commets, boring, just read println messages
    -switched to MIT license