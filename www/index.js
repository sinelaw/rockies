import { Game, Cell } from "rockies";
import { memory } from "rockies/rockies_bg.wasm";

const canvas = document.getElementById("the-canvas");

const SIZE = 64;
const CELL_SIZE = Math.min(canvas.clientWidth / SIZE, canvas.clientHeight / SIZE) | 0; // px


const game = Game.new(SIZE, SIZE);
const width = game.width();
const height = game.height();

const ticks = document.getElementById("ticks");
const cells_count = document.getElementById("cells-count");
const collisions_count = document.getElementById("collisions-count");
const collision_pairs_tested = document.getElementById("collision-pairs-tested");

canvas.height = (CELL_SIZE) * height + 1;
canvas.width = (CELL_SIZE) * width + 1;

const ctx = canvas.getContext('2d');


const heldKeys = new Set();


document.onkeydown = (e) => {
    game.key_down(e.key);
};
document.onkeyup = (e) => {
    game.key_up(e.key);
};


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

canvas.ontouchmove = (e) => {
    e.preventDefault();
    let x = e.touches[0].clientX - canvas.offsetLeft;
    let y = e.touches[0].clientY - canvas.offsetTop;
    game.click(x / (CELL_SIZE + 1), y / (CELL_SIZE + 1));
};
