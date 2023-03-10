<script setup lang="ts">
import { ref, reactive } from "vue";
import iconButton from "./iconButton.vue";
import MedianCut from "mediancut";

const imgUrl = ref("");
const post_thumb_mes = ref("");

function BlobToImageData(blob: Blob): Promise<ImageData> {
  let blobUrl = URL.createObjectURL(blob);
  return new Promise((resolve, reject) => {
    let img = new Image();
    img.onload = () => resolve(img);
    img.onerror = (err) => reject(err);
    img.src = blobUrl;
  }).then((img:any) => {
    URL.revokeObjectURL(blobUrl);
    let [w, h] = [img.width, img.height];
    let canvas = document.createElement("canvas");
    canvas.width = w;
    canvas.height = h;
    let ctx = canvas.getContext("2d");
    ctx?.drawImage(img, 0, 0);

    return ctx.getImageData(0, 0, w, h); // some browsers synchronously decode image here
  });
}

function ImageDataToBlob(imageData: ImageData): Promise<Blob> {
  let canvas = document.createElement("canvas");
  canvas.width = imageData.width;
  canvas.height = imageData.height;
  canvas.getContext("2d").putImageData(imageData, 0, 0);

  return new Promise((resolve, reject) => {
    canvas.toBlob(resolve, "image/svg"); // implied image/png format
  });
}

const emit = defineEmits(["onChanged"]);
function _emit(b64: string) {
  emit("onChanged", b64);
}

function blobToBase64(blob: Blob): Promise<string> {
  return new Promise((resolve, _) => {
    const reader = new FileReader();
    reader.onloadend = () => resolve(reader.result);
    reader.readAsDataURL(blob);
  });
}

async function setImage(blobOrig: Blob) {
  if (blobOrig == null) {
    imgUrl.value = "";
    post_thumb_mes.value = "Image not found in clipboard.";
    _emit("");
    return;
  }

  const imagedata = await BlobToImageData(blobOrig); //.then((imagedata) => {
  // 減色
  let medianCut = new MedianCut(imagedata);
  let iData = medianCut.reduce(16);
  const blob = await ImageDataToBlob(iData);

  // blobを処理
  blobToBase64(blob).then((b64) => {
    imgUrl.value = URL.createObjectURL(blob);

    post_thumb_mes.value = `Type : ${blob.type},  Size : ${Math.round(
      blob.size / 1000
    )}kB`;
    _emit(b64);
  });
  //});
  // });
}

function paste_image() {
  post_thumb_mes.value = "";

  navigator.clipboard
    .read()
    .then((clipItems: ClipboardItems) => {
      const clipItem = clipItems[0];
      const type = clipItem.types.filter((s) => s.startsWith("image/"))[0]; // ex. "image/png"
      if (type == undefined) {
        setImage(null);
        console.log(clipItem);
        return;
      }

      clipItem.getType(type).then((blob: Blob) => {
        setImage(blob);
      });
    })
    .catch((e) => {
      if (e instanceof DOMException) {
        post_thumb_mes.value = e.message;
      } else console.log(e);
    });
}
</script>

<template >
  <div>
    <div class="image-input-container">
      <button class="thumbnail-button shadow" @click="paste_image">
        Paste Image from clipboard
      </button>
      <iconButton
        caption="Clear"
        icon="img/clear.png"
        @click="setImage(null, '')"
      />
    </div>
    <div class="post-input">
      <div>{{ post_thumb_mes }}</div>
      <img class="post-thumbnail-img" :src="imgUrl" />
    </div>
  </div>
</template>

<style>
.image-input-container {
  display: flex;
  gap: 10px;
}
</style>
