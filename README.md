# 🌀 SpiralCity 🌀

*Minimalist 2D city builder / rogue-lite prototype written in Rust.*

## 🧠 Concept

SpiralCity is a turn-based city builder played on a growing spiral grid.

Each turn:
- the player is offered a small set of buildings
- chooses one of them
- the building is automatically placed in the next position in the spiral
- resources are produced based on the current city layout, with some of them affecting nearby buildings

The game ends when none of the proposed buildings can be built due to lack of resources.

## V0

### 🏗️ Buildings (V0)
- 🏠 House — increases population
- 🌲 Forest — produces wood (requires population)
- 🪨 Quarry — produces stone (requires population)
- 🪚 Workshop — boosts production of adjacent buildings (requires resources)

For now, the game is very imbalanced

### 📦 Resources
- 👥 Population (occupied / total)
- 🌲 Wood
- 🪨 Stone

Population is required to operate production buildings.

### 🖥️ Interface

The game currently runs entirely in the terminal.

The city is displayed as an ASCII/emoji grid, one cell per building.

## 🎯 Current status

It's V0: functionnal prototype:   
✔️ Core game loop implemented   
✔️ Turn system and resource production   
✔️ Spiral grid placement   
❌ No balance   
❌ No objectives or progression   
❌ No graphical interface   

## 🚧 Next steps (V1)
- Improve balance (population bottleneck, resource overflow)
- Display building stats during choices
- Refactor code into modules for clarity
- Improve debug and production visibility

## 🛠️ Tech

Language: Rust   
No game engine   
Terminal-based rendering