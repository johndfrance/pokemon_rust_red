# pokemon_rust_red
I am currently a student in the Computer Programming and Analysis program at Durham College in Oshawa, Ontario, going into my
second year. As a summer project I wanted to try to get as much experience as I could with the Rust programming language, and 
building this ended up being my main focus for practicing. 


THINGS TO KNOW BEFORE YOU PLAY:
-From anywhere in the Overworld the player can input '9' to open the Menu and then '4' to Save the game. This creates
a JSON file in the same folder that the EXE file is in which contains the saved game's data. 

-The game only goes as far as Mt. Moon. 

-Moves that affect the user's stats are not implemented, so do not count on these. Moves that affect the opponent's stats
should all work.  

-Moves that induce freezing, confusion, or paralysis have not been implemented, moves that cause burning and 
poison should work correctly, and moves that cause sleeping work only in Trainer battles. 

-For moves that are an attack plus a chance at an additional effect, the additional effect is not implemented, only
the regular attack.

-There are no regular items in the game, in wild battles selecting 'Items' will automatically throw a pokeball. In trainer
battles it will just prompt the player to choose a different option. You cannot heal pokemon in battle, only at pokemon 
centres. 

-Selecting PokeDex from the Menu will crash the game and you will lose any unsaved data. 

-The game does not stop you from depositing all your pokemon into the PC, but if you do this and get into a battle
the game will crash. This and the above are the only known causes of crashing, but there are likely others so 
save frequently to avoid lossing progress. 


REFLECTIONS:
In building this I did achieve most of the goals I set for myself with a few exceptions. 
First are the things I'm most happy with:

-Most basic functionality of the pokemon game was achieved. Damage is calculated based on the move and the stats of the 
pokemon in the battle, pokemon faint when they have zero hp and the battle continues until one side has no healthy pokemon. 
Multiple stats affecting moves and status affecting moves where successfully implimented. You can run from wild battles and 
catch wild pokemon. Players can have up to 6 pokemon on their team and there is a box they can store additional pokemon in. 
The world can be navigated and the player encounters both quest tasks and battles both wild and with other trainers. 

-Getting the game to save was a major goal achived. I wanted there to be at least enough game that I could get friends to 
play it, and being able to save the game and load their save file was achieved. 

-A major hurdle was getting the game to play on different machines. When I first tried it on a second computer all the colours
I had used were appearing as ANSI escape codes, but I was able to use Crossterm to fix this eventually. 

Problems and disapointments:

-Currently there are two battle functions, one for trainer battles and one for wild battles. I did this because the trainer
battle takes in two trainers, each with teams of pokemon, while the wild function accepts just the singular bare wild pokemon.
I contemplated making a dummy empty trainer who would be the owner of all the wild pokemon, but I couldn't work this out. I am
certain there is some way to make this work in an elegant way using generics and traits, but I was never able to make use of 
generics in this program successfully. 

-I ran out of time to make the PokeDex. I'm disappointed in this because I basically had a worked out idea for how to implement
it but I just ran out of time. The PokeDex option in the menu is the only remaining 'TODO' in the game. 

-I was never able to figure out how to structure Items. I assume this would have required building a series of traits
and then composing them into the different item categories, but it was just too much for me at this point. 

-I only realized very late that there was going to be no organic way to include moves that affect the user's stats, 
and this was disapointing because there was a lot of moves that have that sort of effect. 

-I wanted a more organized system for NPC dialogue, but that never panned out. 

