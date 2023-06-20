let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

export function drawFrame(x, y, width, height, buffer) {
    let imageData = ctx.createImageData(width, height);
    // TODO: Fill buffer with image data from the buffer
    for(let i = 0; i < imageData.data.length; i += 4) {
        imageData.data[i + 0] = 190;
        imageData.data[i + 1] = 0;
        imageData.data[i + 2] = 210;
        imageData.data[i + 3] = 255;
    }

    ctx.putImageData(imageData, x, y);
}