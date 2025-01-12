# Installation instructions
Binary releases are available for Windows.<br>
If you use another platform, you will need to compile the program yourself.<br>
**Using Binary Releases**
1. Download the matching zip for your architecture.
2. Unzip all files inside to the location from which you'd like to run the game.

**Compilation Instructions**
1. Install the dependencies and the Rust toolchain for your platform as described [here](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies).
2. Clone this repository
3. Run `cargo build --release`
4. Find your target folder in the `target` folder
5. Copy the contents of your architectures folder, as well as the `assets` folder from the repository's root, to a new folder, from which you'd like to run the game.

**Sharing Levels**<br>
The level data is saved in the file `assets/level.json`. Send this file to the person you'd like to share the level with and have them drag it into their `asset` folder.<br>
**THIS WILL OVERWRITE THEIR CURRENT LEVEL. MAKE SURE TO BACK UP YOUR LEVEL FILE BEFORE OVERWRITING, IF YOU WANT TO KEEP IT.**
