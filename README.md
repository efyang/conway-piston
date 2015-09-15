# conway-piston
Conway's Game of Life using Piston

### Controls:
* Esc => Exit
* R => Reset to previous seed
* G => New random seed
* S => Save Seed
* V => Save current value
* C => Delete all saved seeds
* E => Edit mode
* P => Pause
* N => Normal/Default mode

### Todo: 
* ~~Change all vectors to arrays to take advantage of stack vs heap allocation~~ all values in rust are default stack allocated
* ~~Parallel Checking~~
* ~~Saving and reading seeds~~
* ~~Seed editing mode~~
* ~~Commandline arguments for starting mode~~

Uses MattWoelk's snake-piston and xairy's rust-sudoku as a general template (piston examples aren't very good)
