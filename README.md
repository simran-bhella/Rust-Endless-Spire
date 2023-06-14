# 23Spring-410P-Endless Spire

## Team members

    Bruce Truong, brucet@pdx.edu
    
    Chentao Ma, chentao@pdx.edu
    
    John Asencio-Giron, jaa23@pdx.edu
    
    Shihao Li, lishih@pdx.edu
    
    Simranjit Bhella, sbhella@pdx.edu

## Brief Introduction
"Endless Spire", our game will involve navigating a maze, avoiding enemies, and climbing towers while being patrolled by enemies. The player will input commands in order to control their avatar sprite. Navigating through each map, the player will have only three healths
to make it to the top. To get to the next level the player must make it to the staircase at the other side of the map. If the player comes
into contact with an enemy, they will lose one health. 

## Current deliverables
1. A complete Roguelike game developed using Rust programming language.
	
    MVP:
    - Player Movement + Sprites w/ directional movement
    - Enemies (Randomized Movement)
	- 5-level map
	- Win Condition (Reaching the top)
    - Lose Condition (Health <=0 || Time expires [steps || real time])
	- Stairs functionality

	Stretch Goals:
    * Items (ie: bombs)
    * 10 maps/levels
    * Maps are generated procedurally
    * Keys and door
    * Enemies use pathing algorithm (https://arongranberg.com/astar/ , https://en.wikipedia.org/wiki/A*_search_algorithm)
    * Multiplayer
    * Game save
    * Localization


## Problems we ran into
1. GitLab
    - At first, we struggled to set up how to work on GitLab, getting our keys set up then 
    also figuring out how to push and merge without messing up others progress.
2. Ggez
    - We ran into a problem where no matter what we did we could not display an image. Every 
    place we looked did not help. Finally with the help of our beloved TA Nicholas we found 
    that the problem was actually the version of Ggez we were using. Our code was actually right,
    but the version of Ggez we were using had a bug. We then had to switch the version of Ggez we 
    were using and thankfully we were still in the beginning of development so it did not interfere
    with some of the other progress we made. 
3. Timeline
    - Balancing life, and other class work ontop of a project like this was very difficult. About 
    half of our deadlines we had to push back because various people were busy with other classes 
    and/or life. This made the project a little bit more stressful because of the fact we were 
    uncertain if we would reach the MVP. 

## Gameplay Screenshots
# Starting Screen
![Alt text](/resources/ss.png "Starting Screen")

# Second Level
![Alt text](/resources/map2.png "Map 2")

## Installation/Running
Installing is easy! First make sure you have Rust installed on your system, then just download the source code from https://gitlab.cecs.pdx.edu/lishih/23spring-410p-endless-spire and unzip. Start the game with
'''
`cargo run`

'''`

## Testing part:
Since we are using Ggez framework to build our game, we haven't found any specific functions or relavant crates about testing code.
However, we found those two ways are we prefer the most.

*Unit Testing*: Writing test cases to verify that a single feature, class, or module in a game is working as expected. Tests can be run using a testing framework in Rust, such as the 
cargo test command. Make sure the test covers key features in the game, including map generation, movement logic, enemy AI, and more.
Like what we did in previous assignments, we can run cargo test, but unluckily, we found it's hard to implement the test in this way. Due to the time limit, it seems tough to deal with the syntax issues. Besides, we don't see the meaning of unit testing in our game since during the developing process, we always keep modifying and testing code by running our 
game and check the difference from modification. However, we still include a unit test template in the src/test.rs just in case. 

**Visual Testing**: Visual effects are usually an important part of Roguelikes. We need to make sure the game's graphics and interface elements work properly at various resolutions and display Settings. We can run the game manually and check that the game interface, map generation, image rendering, etc., are correct. (It is also possible to use automated tools such as SikuliX or Selenium to take screenshots of the game interface and compare them.)
In our `src/test.rs`, we mocked a completely new game and change the corresponding code to test the different results. Such as testing the enemies movements by checking if the enemies 
freeze in the test. What's more, in our test, it includes the tests on such as map bounds, tile's images, health, collision, stairs and startscreen. 
We developers can test the corresponding functions by playing the game and manipulating the character in different areas.
To run our test, instead, run the command : ` cargo run --bin test`. We think this is the best and the most effiecient way to test functionality.
 


## Support

- If you run into any problems feel free to contact any of the creators:
    * Bruce Truong, brucet@pdx.edu
    * Chentao Ma, chentao@pdx.edu
    * John Asencio-Giron, jaa23@pdx.edu
    * Shihao Li, lishih@pdx.edu
    * Simranjit Bhella, sbhella@pdx.edu

## Roadmap
This is version V1, in the future there might be versions that implement more features to the game.

## Resources
Background sourced from Ansimuz on opengameart.com - https://opengameart.org/content/phantasy-dungeon-entrance

Sprite sourced from Anifarea on opengameart.com - https://opengameart.org/content/antifareas-rpg-sprite-set-1-enlarged-w-transparent-background-fixed

Examples and block and tile sources from Ggez github - https://github.com/ggez/ggez/tree/master

## License
Background & Sprites - CC0 1.0 Universal (CC0 1.0)
Public Domain Dedication

Ggez Examples - The MIT License (MIT)

Copyright (c) 2016-2017 ggez-dev

This work is licensed under the "MIT License". Please see the file LICENSE.txt in this distribution for license terms.