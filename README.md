# gifenc-rs

WebAssembly GIF converter (callable from TypeScript) built from rust image crate.



# Demo

https://lilacs2039.github.io/rust-gif-wasm



# npm

https://www.npmjs.com/package/rust-gif-wasm



# Install

```bash
npm install gifenc-rs
```





# Usage

## 1. Copy `*_bg.wasm`

Copy `*_bg.wasm` from `node_modules/gifenc-rs/*` to `./assets/*`.



## 2. import&call function

```vue
<script setup lang="ts">
import gif_wasm_init, { encode_gif } from "gifenc-rs";
import gif_wasm_module from "./assets/gifenc_rs_bg.wasm?url";
    
// init wasm
gif_wasm_init(gif_wasm_module)

// ...
// convert
const buf: Uint8Array = encode_gif(buffer as Uint8Array);

```


<details>
proper example


```vue
<script setup lang="ts">
import gif_wasm_init, { encode_gif } from "gifenc-rs";
import gif_wasm_module from "./assets/gifenc_rs_bg.wasm?url";
    
// init wasm
gif_wasm_init(gif_wasm_module)

function encode(dataURL: string) {
  const bin = window.atob(dataURL.replace(/.+,/, ""));
  var buffer = new Uint8Array(bin.length);
  for (var i = 0; i < bin.length; i++) {
    buffer[i] = bin.charCodeAt(i);
  }
  const buf: Uint8Array = encode_gif(buffer as Uint8Array);
  // Convert to Blob
  try {
    var blob = new Blob([buf], {
      type: "image/gif",
    });
    url.value = URL.createObjectURL(blob);
    post_thumb_mes.value = `Type : ${blob.type},  Size : ${Math.round(
      blob.size / 1000
    )}kB`;
  } catch (e) {
    console.error(e);
  }
}
</script>

<template></template>
```

</details>



# Development

## Build

```bash
wasm-pack build --target web
```



## publish

```bash
npm publish ./pkg
```









