// https://codepen.io/mdjasper/pen/GYOVqr?editors=1010

const app = lib => {

  // const width = 260, height = 200;
  const width = 640, height = 480;

  const player = document.createElement('video');
  player.autoplay = true;
  // const canvas = document.getElementById('canvas');
  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  const tempCanvas = document.createElement("canvas");
  tempCanvas.width = width;
  tempCanvas.height = height;
  const tempContext = tempCanvas.getContext("2d");
  const context = canvas.getContext('2d');

  const player1 = document.getElementById("player")

  const constraints = {
    video: true,
  };

  const animationFrame = () => {
    // put stream on in-memory canvas and get rgba data out
    tempContext.drawImage(player, 0, 0, canvas.width, canvas.height);
    const data = tempContext.getImageData(0,0,canvas.width,canvas.height).data
    console.log("from js", data.length)
    // pass rgba data to webassembly function
    lib.calculate(data);
    // const newData = lib.get();
    //create new Image and draw it to a canvas
    const newImageData = new ImageData(Uint8ClampedArray.from(data), width, height);
    context.putImageData(newImageData, 0, 0);

    window.requestAnimationFrame(animationFrame);
  }


  navigator.mediaDevices.getUserMedia(constraints)
    .then((stream) => {
      // Attach the video stream to the video element and autoplay.
      player.srcObject = stream;
      window.requestAnimationFrame(animationFrame);
      const canvasStream = canvas.captureStream();
      player1.srcObject = canvasStream;
    });

}

import('./wasm_bindgen_lib')
.then(app)
.catch(console.error);