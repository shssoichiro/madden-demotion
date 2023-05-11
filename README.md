## How do I use this?

IMPORTANT. When starting this, you should be in the postseason BEFORE the Super Bowl. The first steps need to be run BEFORE exporting Super Bowl week rosters to Neon. Otherwise you bricked it.

Why is this so complicated? MADDEN. Also Neon doesn't give us all the info we need without doing this way. Neon's website can show when a player gained or lost a dev trait, but their CSV export doesn't.

1. Go to Neon and select Export CSV on the sidebar
2. Select the current year, Regular/Post Season. Unselect anything from "Week".
3. Hit the Export button at the bottom.
4. Extract the files to a folder called "neon_season"
5. Select "Players" nothing else for the checkboxes at the top, export with the top export button
6. Extract to a folder called "neon_players_old"
7. NOW you can feel free to advance to the Super Bowl, have your users play it, etc. The best time to continue on with the next step is probably the offseason stage when Retirements are announced, that way retired players are not included in the dev trait counts
   - It's fine if this takes a while. The previous files won't go bad or anything even if you're in SB for several days. Just don't lose them.
8. When ready, export rosters from the Madden App to Neon
9. Repeat steps 5-6 but extract to a folder called "neon_players_new"
10. Put those folders full of CSVs in the same folder as this tool
11. Make sure the `THREE_FOUR_TEAMS` list in this code is updated. It should contain all the teams who were running a 3-4 scheme in the season which is ending.
12. Recompile if needed then run this tool
