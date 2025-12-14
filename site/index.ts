import * as wasm from "snake";
import { AppleDelta } from "snake";

async function appleDelta(event : AppleDelta) {
    console.log(event);
}

let game = new wasm.SnakeGame(
    10,
    10,
    appleDelta);


