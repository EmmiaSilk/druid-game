let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

export function image_data_from_bitmap(ctx, bitmap) {
    let imageData = ctx.createImageData(bitmap.width, bitmap.height);
    let colors = bitmap.colors;

    for(let i = 0; i < colors.length; i += 1) {

        let color_u32 = colors[i];
        let a = (color_u32 & 0xFF000000) >> 24;
        let r = (color_u32 & 0x00FF0000) >> 16;
        let g = (color_u32 & 0x0000FF00) >> 8;
        let b = (color_u32 & 0x000000FF) >> 0;

        imageData.data[i*4+0] = r;
        imageData.data[i*4+1] = g;
        imageData.data[i*4+2] = b;
        imageData.data[i*4+3] = 255;
    }

    return imageData;
}
