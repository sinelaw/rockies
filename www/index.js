import { Game, Cell } from "rockies";
import { memory } from "rockies/rockies_bg.wasm";

const canvas = document.getElementById("the-canvas");

const SIZE = 64;
const CELL_SIZE = Math.min(canvas.clientWidth / SIZE, canvas.clientHeight / SIZE) | 0; // px


const game = Game.new(SIZE, SIZE);
const width = game.width();
const height = game.height();

const ticks = document.getElementById("ticks");
const version = document.getElementById("version");
const cells_count = document.getElementById("cells-count");
const collisions_count = document.getElementById("collisions-count");
const collision_pairs_tested = document.getElementById("collision-pairs-tested");
const touches = document.getElementById("touches");
const dig_checkbox = document.getElementById("dig-checkbox");

canvas.height = (CELL_SIZE) * height + 1;
canvas.width = (CELL_SIZE) * width + 1;

version.textContent = game.version();

const ctx = canvas.getContext('2d');

const renderLoop = () => {

    game.tick();

    drawPixels();

    let stats = game.stats();

    ticks.textContent = stats.ticks();
    cells_count.textContent = stats.cells_count();
    collisions_count.textContent = (stats.collisions_count() / stats.ticks()) | 0;
    collision_pairs_tested.textContent = (stats.collision_pairs_tested() / stats.ticks()) | 0;



    requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawPixels = () => {
    const pixelsPtr = game.pixels();
    const pixels = new Uint32Array(memory.buffer, pixelsPtr, width * height);

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            ctx.beginPath();

            let val = pixels[idx];
            ctx.fillStyle = "#" + val.toString(16).padStart(6, "0");
            //console.log("[%d,%d] = %s = %s", row, col, pixels[idx].toString(16), ctx.fillStyle);

            ctx.fillRect(
                col * CELL_SIZE + 1,
                row * CELL_SIZE + 1,
                CELL_SIZE,
                CELL_SIZE
            );

            ctx.stroke();
        }
    }

};

drawPixels();
requestAnimationFrame(renderLoop);

canvas.onmousemove = (e) => {
    if (e.buttons > 0) {
        game.click(e.offsetX / (CELL_SIZE + 1), e.offsetY / (CELL_SIZE + 1));
    }
};

canvas.onclick = (e) => {
    game.click(e.offsetX / (CELL_SIZE + 1), e.offsetY / (CELL_SIZE + 1));
};

canvas.addEventListener('blur', function (event) {
    game.unfocus();
});

window.addEventListener('blur', function (event) {
    game.unfocus();
});

let is_shift_down = () => {
    return dig_checkbox.checked;

};

// Helper function to find buttons by key
const findButtonsByKey = (key) => {
    return Array.from(document.querySelectorAll('.move-button, .move-button-checkbox')).filter(el => {
        const keys = (el.dataset.keys || '').split(',');
        return keys.includes(key.toLowerCase());
    });
};

// Helper functions to activate/deactivate buttons
const activateButton = (button) => {
    if (button.classList.contains('move-button-checkbox')) {
        if (!button.dataset.keys) return; // Skip buttons without key mappings
        const keys = button.dataset.keys.split(',');
        keys.forEach(key => {
            if (pressedKeys.has(key)) {
                button.checked = true;
            }
        });
        // Trigger the change event so handlers are notified
        button.dispatchEvent(new Event('change'));
    } else {
        button.classList.add('active');
    }
};

const deactivateButton = (button) => {
    if (button.classList.contains('move-button-checkbox')) {
        if (!button.dataset.keys) return; // Skip buttons without key mappings
        const keys = button.dataset.keys.split(',');
        keys.forEach(key => {
            if (!pressedKeys.has(key)) {
                button.checked = false;
            }
        });
        // Trigger the change event so handlers are notified
        button.dispatchEvent(new Event('change'));
    } else {
        button.classList.remove('active');
    }
};

// Track currently pressed keys
const pressedKeys = new Set();
let shiftToggled = false;

// Helper function to normalize key names
const normalizeKey = (key) => {
    key = key.toLowerCase();
    switch (key) {
        case 'arrowup': return 'w';
        case 'arrowdown': return 's';
        case 'arrowleft': return 'a';
        case 'arrowright': return 'd';
        case ' ': return ' ';
    }
    return key;
};

const findButtonElements = () => {
    return Array.from(document.querySelectorAll('.move-button, .move-button-checkbox'));
};

// Update all button states based on currently pressed keys
const updateAllButtonStates = () => {
    const elems = findButtonElements();

    elems.filter(el => {
        const buttonKeys = (el.dataset.keys || '').split(',').map(k => normalizeKey(k));
        // For combination buttons (e.g. w+a)
        if (buttonKeys.every(key => pressedKeys.has(key))) {
            activateButton(el);
        } else {
            deactivateButton(el);
        }
    });
};

// Handle keyboard events
document.onkeydown = (e) => {
    const key = normalizeKey(e.key);
    pressedKeys.add(key);
    touches.textContent = key;
    game.key_down(key);
    updateAllButtonStates();
};

document.onkeyup = (e) => {
    const key = normalizeKey(e.key);
    pressedKeys.delete(key);
    touches.textContent = key;
    game.key_up(key);
    updateAllButtonStates();
};

// Clear pressed keys when window loses focus
window.onblur = () => {
    pressedKeys.clear();
    updateAllButtonStates();
};

// Setup control buttons
const setupControlButtons = () => {
    // Special handling for shift checkbox
    const toggleButtons = document.querySelectorAll('.move-button-checkbox');
    toggleButtons.forEach(checkbox => {
        if (!checkbox.dataset.keys) return; // Skip buttons without key mappings
        const keys = checkbox.dataset.keys.split(',');
        if (keys.includes('shift')) {
            const handleChange = () => {
                if (checkbox.checked) {
                    pressedKeys.add('shift');
                    game.key_down('shift');
                } else {
                    pressedKeys.delete('shift');
                    game.key_up('shift');
                }
            };

            checkbox.addEventListener('change', handleChange);
            return; // Skip regular event binding for shift button
        }
    });

    const buttons = document.querySelectorAll('.move-button');

    buttons.forEach(button => {
        if (!button.dataset.keys) return; // Skip buttons without key mappings
        const keys = button.dataset.keys.split(',');

        console.log("Button: ", button, "Keys: ", keys);
        const pressKeys = () => keys.forEach(key => game.key_down(key));
        const releaseKeys = () => keys.forEach(key => game.key_up(key));

        // Mouse events
        button.addEventListener('mousedown', pressKeys);
        button.addEventListener('mouseup', releaseKeys);
        button.addEventListener('mouseleave', releaseKeys);

        // Touch events
        button.addEventListener('touchstart', (e) => {
            e.preventDefault(); // Prevent double-firing on mobile
            pressKeys();
        });
        button.addEventListener('touchend', (e) => {
            e.preventDefault();
            releaseKeys();
        });
        button.addEventListener('touchcancel', (e) => {
            e.preventDefault();
            releaseKeys();
        });

        // Blur event (when button loses focus)
        button.addEventListener('blur', releaseKeys);
    });


};

// Initialize controls
setupControlButtons();
