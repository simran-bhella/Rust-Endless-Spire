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
![Alt text](/resources/sss.png "Starting Screen")

## Installation/Running
Installing is easy! Just download the source code from https://gitlab.cecs.pdx.edu/lishih/23spring-410p-endless-spire and unzip. Then open in a terminal or vscode and type cargo run.

## Support
If you run into any problems feel free to contact any of the creators:
    -Bruce Truong, brucet@pdx.edu
    -Chentao Ma, chentao@pdx.edu
    -John Asencio-Giron, jaa23@pdx.edu
    -Shihao Li, lishih@pdx.edu
    -Simranjit Bhella, sbhella@pdx.edu

## Roadmap
This is version V1, in the future there might be versions that implement more features to the game.

## Resources

Sprite sourced from Anifarea on opengameart.com - https://opengameart.org/content/antifareas-rpg-sprite-set-1-enlarged-w-transparent-background-fixed

Examples and block and tile taken from Ggez github - https://github.com/ggez/ggez/tree/master

## License
Sprites - CC0 1.0 Universal (CC0 1.0)
Public Domain Dedication

Ggez Examples - The MIT License (MIT)

Copyright (c) 2016-2017 ggez-dev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
