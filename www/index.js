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

function grid_index_name(grid_index) {
    let js = grid_index.to_js();
    return `${js.grid_offset.x}_${js.grid_offset.y}`
}

// IndexedDB helpers
function openDB() {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open('rockies-db', 1);
        request.onupgradeneeded = function (event) {
            const db = event.target.result;
            if (!db.objectStoreNames.contains('grids')) {
                db.createObjectStore('grids');
            }
        };
        request.onsuccess = function (event) {
            resolve(event.target.result);
        };
        request.onerror = function (event) {
            reject(event.target.error);
        };
    });
}

function idbSet(key, value) {
    return openDB().then(db => {
        return new Promise((resolve, reject) => {
            const tx = db.transaction('grids', 'readwrite');
            const store = tx.objectStore('grids');
            const req = store.put(value, key);
            req.onsuccess = () => resolve();
            req.onerror = (e) => reject(e);
        });
    });
}

function idbGet(key) {
    return openDB().then(db => {
        return new Promise((resolve, reject) => {
            const tx = db.transaction('grids', 'readonly');
            const store = tx.objectStore('grids');
            const req = store.get(key);
            req.onsuccess = () => resolve(req.result);
            req.onerror = (e) => reject(e);
        });
    });
}

async function loadAndSave() {

    // Load grids from IndexedDB
    let grids_to_load = game.get_missing_grids();
    for (const grid_index of grids_to_load) {
        console.log("loading grid: " + grid_index_name(grid_index));
        const grid = await idbGet(`grid_${grid_index_name(grid_index)}`);
        if (grid) {
            game.load_grid(grid_index, grid);
        } else {
            game.generate_grid(grid_index);
        }
    }

    let loaded_grids = game.get_loaded_grids();
    const savePromises = [];
    for (const grid_index of loaded_grids) {
        console.log("saving (dropping) grid: " + grid_index_name(grid_index));
        const grid = game.save_grid(grid_index);
        if (grid) {
            // Collect promises instead of awaiting
            savePromises.push(idbSet(`grid_${grid_index_name(grid_index)}`, grid));
        }
    }
    let droppable_grids = game.get_droppable_grids();
    for (const grid_index of droppable_grids) {
        console.log("dropping grid: " + grid_index_name(grid_index));
        game.drop_grid(grid_index);
    }

    // Wait for all saves to complete
    await Promise.all(savePromises);

}

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


function resave() {
    loadAndSave().then(() => {
        setTimeout(resave, 5000);
    });
}
resave();
renderLoop();


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
