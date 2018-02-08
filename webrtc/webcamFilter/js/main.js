//Variables globales
let width = 500,
    height = 0,
    filter = 'none',
    streaming = false;

//Elementos del DOM
const video = document.getElementById('video');
const canvas = document.getElementById('canvas');
const photos = document.getElementById('photos');
const clearButton = document.getElementById('clear-button');
const photoButton = document.getElementById('photo-button');
const photoFilter = document.getElementById('photo-filter');

//Obtener media
navigator.mediaDevices.getUserMedia({video: true, audio: false})
.then(function(stream) {
  //Ligar con la etiqueta del video
  video.srcObject = stream;
  //Mostrar video
  video.play();
})
.catch(function(err) {
  console.log(`Error $(err)`)
});

//Mostrar cuando este listo
video.addEventListener('canplay', function(e) {
  if(!streaming) {
    height = video.videoHeight/ (video.videoWidth / width)

    video.setAttribute('width', width);
    video.setAttribute('height', height);
    canvas.setAttribute('width', width);
    canvas.setAttribute('height', height);

    streaming = true;
  }
}, false);

photoButton.addEventListener('click',function(e) {
  takePicture();

  e.preventDefault();
}, false);

//boton limpiar
clearButton.addEventListener('click',function(e) {
  photos.innerHTML = '';
  filter = 'none';
  video.style.filter = filter;
  photoFilter.selectedIndex = 0;
}, false);

//Filtros
photoFilter.addEventListener('change',function(e) {
//Cambiando el filtro a la opción escogida
  filter = e.target.value;
  video.style.filter = filter;
  e.preventDefault();
}, false);

//Poniendo la foto del canvas al
function takePicture() {
  const context = canvas.getContext('2d');
  if(width && height) {
    canvas.width = width;
    canvas.height = height;
//Dibujar la imagen en el canvas
    context.drawImage(video,0,0,width,height);
    const imgURL = canvas.toDataURL('image/png');
//Creando una imagen
    const img = document.createElement('img');
//Set img source
    img.setAttribute('src',imgURL);
//Aplicando el filtro
    img.style.filter = filter;
    photos.appendChild(img);
  }
}
