import { Universe, Cell } from "rockies";
import { memory } from "rockies/rockies_bg.wasm";

const SIZE = 64;
const CELL_SIZE = (0.7 * Math.min(document.body.clientWidth, document.body.clientHeight) / SIZE) | 0; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";


const universe = Universe.new(SIZE, SIZE);
const width = universe.width();
const height = universe.height();

const ticks = document.getElementById("ticks");
const cells_count = document.getElementById("cells-count");
const collisions_count = document.getElementById("collisions-count");
const collision_pairs_tested = document.getElementById("collision-pairs-tested");

const canvas = document.getElementById("the-canvas");

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');


const renderLoop = () => {

    universe.tick();

    drawGrid();
    drawPixels();

    let stats = universe.stats();

    ticks.textContent = stats.ticks();
    cells_count.textContent = stats.cells_count();
    collisions_count.textContent = (stats.collisions_count() / stats.ticks()) | 0;
    collision_pairs_tested.textContent = (stats.collision_pairs_tested() / stats.ticks()) | 0;



    requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawPixels = () => {
    const pixelsPtr = universe.pixels();
    const pixels = new Uint32Array(memory.buffer, pixelsPtr, width * height);



    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            ctx.beginPath();

            let val = pixels[idx];
            ctx.fillStyle = "#" + val.toString(16).padStart(6, "0");
            //console.log("[%d,%d] = %s = %s", row, col, pixels[idx].toString(16), ctx.fillStyle);

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );

            ctx.stroke();
        }
    }

};



drawGrid();
drawPixels();
requestAnimationFrame(renderLoop);

canvas.onmousemove = (e) => {
    if (e.buttons > 0) {
        universe.click(e.offsetX / (CELL_SIZE + 1), e.offsetY / (CELL_SIZE + 1));
    }
};

canvas.onclick = (e) => {
    universe.click(e.offsetX / (CELL_SIZE + 1), e.offsetY / (CELL_SIZE + 1));
};

canvas.ontouchmove = (e) => {
    e.preventDefault();
    let x = e.touches[0].clientX - canvas.offsetLeft;
    let y = e.touches[0].clientY - canvas.offsetTop;
    universe.click(x / (CELL_SIZE + 1), y / (CELL_SIZE + 1));
};