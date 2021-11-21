# map-game-board-rs
When complete, this should be able to represent any game board that 
is based on a geographical map. The map will be defined with a YAML
file and an SVG file for display.  Hopefully there will be a community
effort to create many great map files, enabling these games to take
on many forms.

The original purpose is for a Risk-like game, but it is envisioned
that other games will be possible using the same code and maps, so it is
being written as generically as possible.

Your game must implement the BoardSpot trait for whatever type
of data is to be placed in each map territory.

## License

This code may be distributed and used under the GNU General Public License
version 3 or, at your option, any later version published by the
Free Software Foundation.

Copyright to code is retained by each contributor, who agrees to license
it as noted above.  It is intended
that relicensing not be possible.  We hope this will be the basis for many
Free games in the future.
