# Felix
A Discord bot written in Rust. The name comes from iron (Fe) oxide (rust.) <br/>
It is written to be a useful tool for people who want to learn Japanese with other people.
## Commands
- help - As you may have guessed, shows help text for one or all commands.
- strokes - Shows a graphic with the stroke order for a given Japanese character.
- jisho - Searches [Jisho](http://jisho.org) for whatever you tell it to.
- random - Grabs a random word from [Jisho](http://jisho.org) of whatever JLPT level is specified.

## TODO, in order of priority*
- Cache Jisho search results to avoid making unnecessary HTTP requests.
- Add !quiz command, which will be a pictionary style vocabulary quiz on random Japanese words.
- Add role checks for commands
- Fix rogue commas in some definitions in Jisho results.
- Make the words more distinguishable from the readings in Jisho results.

*not necessarily in order of completion

