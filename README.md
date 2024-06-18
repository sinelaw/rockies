# Rockies

Rockies is a simple physics simulation written in Rust and WebAssembly, designed to run in a web browser. It simulates a 2D world with gravity, collisions, and user interaction.

## Features

* **Gravity:** Objects are affected by gravity, pulling them downwards.
* **Collisions:** Objects collide with each other and with walls, bouncing off with realistic physics.
* **User Interaction:** Users can interact with the simulation by clicking and dragging objects, and by controlling a player character.
* **WebAssembly:** The simulation is compiled to WebAssembly, allowing it to run in any modern web browser.

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/sinelaw/rockies.git
   ```

2. **Build the project:**

   ```bash
   cd rockies
   cargo build
   wasm-pack build
   npm run start 
   ```

3. **Run the simulation:** Open http://localhost:8080/ in your web browser.

# Usage

- Click and drag: Click and drag objects to move them around.
- Control the player: Use the arrow keys to move the player character.
- Add cells: Click on the canvas to add new cells.
