let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

export function image_data_from_bitmap(ctx, bitmap) {
    let imageData = ctx.createImageData(bitmap.width, bitmap.height);
    let colors = bitmap.colors;
    for(let i = 0; i < colors.length; i += 1) {
        let color_u32 = colors[i];
        let a = (color_u32 / 0x1000000) % 256;
        let r = (color_u32 / 0x10000) % 256;
        let g = (color_u32 / 0x100) % 256;
        let b = (color_u32 / 0x1) % 256;

        imageData.data[i*4+0] = r;
        imageData.data[i*4+1] = g;
        imageData.data[i*4+2] = b;
        imageData.data[i*4+3] = a;
    }

    return imageData;
}
