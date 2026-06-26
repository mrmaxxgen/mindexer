# mindexer
a simple file indexer written in Rust

mrmaxxgen (c) 2026, licensed under the GNU General Public License v3 <https://www.gnu.org/licenses/>

mindexer is a file indexer/sorter utility for automatic by-type file sorting/indexing in specific folder, it can be used for servers html's pages or just for tidying up your system with zero effort, mindexer handles (almost) everything by itself thru fs/env calls and error handling, it will also help you creating a base to start using it.

current version: 0.0.2 ALPHA (pre-release)

for future releases:
    -configure Makefile
    -fix issues (marked in main.rs)
    -check licensing

changelog:

0.0.1 ALPHA (pre-release):
    -created repo and code base
    -coded main code, not working, needs fix
    -implemented mp3 and mp4 support for now

0.0.2 ALPHA (pre-release):
    -ditched commands and used rust fs/env libraries instead
    -commented code more
    -made code lighter