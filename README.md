## How the fuck do I use this?

IMPORTANT. When starting this, you should be in the postseason BEFORE the Super Bowl. The first steps need to be run BEFORE exporting Super Bowl week rosters to Neon. Otherwise you bricked it.

Why is this so complicated? MADDEN. Also Neon doesn't give us all the info we need without doing this way. Neon's website can show when a player gained or lost a dev trait, but their CSV export doesn't.

1. Go to Neon and select Export CSV on the sidebar
2. Select the current year, Regular/Post Season. Unselect anything from "Week".
3. Select "Players" and nothing else for the checkboxes at the top.
4. Export, extract the files to a folder called "neon_season"
5. Repeat but with Players and Games checked, extract to a folder called "neon_players_old"
6. NOW if you haven't advanced to the Super Bowl, do it, then export rosters from the Madden App to Neon
   - If you have to wait for this, it's fine. The previous files won't go bad or anything.
7. Repeat steps 2-4 and have Players and Games checked but extract to a folder called "neon_players_new"
8. Put those folders full of CSVs in the same folder as this tool
9. Make sure the `THREE_FOUR_TEAMS` list in this code is updated. It should contain all the teams who were running a 3-4 scheme in the season which is ending.
10. Recompile if needed then run this tool
