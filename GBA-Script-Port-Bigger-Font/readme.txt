FFV GBA Script Port - Bigger Font
---------------------------------
This is a simple and humble project that makes the font
seen in battle and when talking bigger.

Please use this patch after you use J121's work here:

https://www.romhacking.net/hacks/3687/

This is only an addendum to that Port, and it won't work
correctly without it!

Thanks to Gens for creating the awesome font used in this,
and for teaching me a lot about fine-tuning the spacing of
letters and symbols.

Thanks to Roak for rewriting GBA Script Port battle dialog to fit with 
larger fonts! Their sibling project to this one provided some guidance 
on how to fix the bug of text running offscreen.

Check it out here:

https://www.romhacking.net/translations/7606/


Thanks to noisecross for creating an awesome Text Editor;
this would not be possible without it!

Read much more about this project here:

https://app.notion.com/p/xj4cks/FFV-GBA-Script-Port-39ede0b6e7514b8892b6fccccb1c0208


-----------------------
Changelog:

25 Sept, 2026: v1.03

Imported Roak's amended battle speech (see link above).
Fixed every "It" (like in Its) versus "lt" (like in melt) typo.
Fixed dozens of spacing typos and some ugly "extra letter" typos.
Fixed "???" monster name.
Fixed icons for Flail, Morn.Star, Ash.
Made the Yes/No option box match the new Gens font exactly.

Added Knight Sword icon to Blood, Flame, Ice, Defender, Excalibur, Ragnarok.

Included compatibility patch for Roak's GBASP Chicago to
- add K.Sword
- add Boomerang
- use Diamond for Blue magic spells



11 Sept, 2026: v1.02

Fixed 8 cases of 'I' showing up instead of "l" in dialogs.
Fixed Bartz's blank name at game start.
Fixed missing space in King Worus dialog.
Returns all Blue spells to original J121 names.



30 May, 2026: v1.01

Fixed the "?" getting replaced with "<>" (diamond icon) issue the Blue Magic menus.



11 February, 2026: v1.00a

Fixed a bug in the opening scene when the automatic dialog would stall 
after Faris spoke until the player pressed "A". This scene is meant to 
be precisely timed with the music so this was extremely suboptimal.