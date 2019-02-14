import * as wasm from "wasm-brickout";
import { memory } from "wasm-brickout/wasm_brickout_bg";

let animationId = null;

const game_state = wasm.SharedState.new();
console.log(game_state);

const renderLoop = () => {
    game_state.update();
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawBall(game_state, ctx);
    drawBricks(game_state, ctx);
    // pre.textContent = universe.render();
    animationId = requestAnimationFrame(renderLoop);
}

window.onload = renderLoop()

// import { CONSTANTS } from "./Constants.js";
export function draw(state, canvas, ctx) {
    [drawBall, drawPaddle, drawBricks].forEach(function(drawFunction) {
        drawFunction({ state, ctx });
    });
    return state;
}

function drawBall( state, ctx ) {
    ctx.fillStyle = "#C6494B";
    ctx.fillRect(
        state.ball_position_x,
        state.ball_position_y,
        state.ball_diameter,
        state.ball_diameter
    );
}

function drawBricks(state, ctx ) {
    const bricksPtr = state.bricks();
    const bricks = new Uint8Array(
        memory.buffer, bricksPtr,  state.columns * state.rows
    );
    console.log(bricks.length);

    ctx.fillStyle = "#ff0000";
    bricks.forEach(function(brick, i) {
        if(brick === 1) {
            ctx.fillRect(
                (i % state.columns) * state.brick_size,
                Math.floor(i / state.rows) * state.brick_size,
                state.brick_size,
                state.brick_size
            );
        }
    });
}

function drawPaddle({ state, ctx }) {
    const { paddlePositionX } = state;

    ctx.fillStyle = "#C6494B";
    ctx.fillRect(
        paddlePositionX,
        500 - CONSTANTS.PADDLE.SIZE.height,
        CONSTANTS.PADDLE.SIZE.width,
        CONSTANTS.PADDLE.SIZE.height
    );
}
