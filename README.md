# GNOSIS
A general purpose productivity TUI app made with [ratatui](https://ratatui.rs/).

## CONTENT
[AGENDA](#AGENDA)
[AGENDA FILE STRUCTURE](#AGENDA FILE STRUCTURE)

### AGENDA

An agenda inspired by [lazyorg](https://github.com/HubertBel/lazyorg)

#### AGENDA FILE STRUCTURE

Activites are stored in a .txt file in a CSV-like syntax. Each line represents one activity ```TITLE;START;END;DESCRIPTION;PRIORITY```
```START``` and ```END``` are saved as UNIX timestamps, and adapt automatically to the local timezone using the ```chrono``` crate.
```PRIORITY``` is an enum with the possible values of : ```LOW```, ```NORMAL```, ```IMPORTANT```


## License

Copyright (c) Rodrigue Gaspard <rodriguegaspard@gmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
