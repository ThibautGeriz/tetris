import { TetrisGame, Color } from './pkg';
import { memory } from './pkg/wasm_tetris_bg';

const colors = {
  [Color.None]: '#FFFFFF',
  [Color.Cyan]: '#00FFFF',
  [Color.Yellow]: '#FFFF00',
  [Color.Purple]: '#800080',
  [Color.Green]: '#008000',
  [Color.Red]: '#FF0000',
  [Color.Blue]: '#0000FF',
  [Color.Orange]: '#FFA500',
};

// // Construct the universe, and get its width and height.
const game = TetrisGame.new();
const columnCount = game.column_count();
const rowCount = game.row_count();
let squareSize = getSquareSize();
const canvas = document.getElementById('tetris-playground');
const context = canvas.getContext('2d');

const fps = new (class {
  constructor() {
    this.fps = document.getElementById('fps');
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = (1 / delta) * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
})();

// resize the canvas to fill browser window dynamically
window.addEventListener('resize', resizeCanvas, false);

function resizeCanvas() {
  squareSize = getSquareSize();
  canvas.width = columnCount * (squareSize + 1);
  canvas.height = rowCount * (squareSize + 1);
}
resizeCanvas();

function getSquareSize() {
  return Math.floor((window.innerHeight - 100) / rowCount);
}

function draw() {
  const squaresPtr = game.squares();
  const squares = new Uint8Array(memory.buffer, squaresPtr, rowCount * columnCount);
  context.beginPath();
  for (let row = 0; row < rowCount; row++) {
    for (let col = 0; col < columnCount; col++) {
      const idx = getIndex(row, col);
      context.fillStyle = colors[squares[idx]];
      context.fillRect(
        col * (squareSize + 1) + 1,
        row * (squareSize + 1) + 1,
        squareSize,
        squareSize,
      );
    }
  }

  context.stroke();
}

function setScore() {
  const score = game.score();
  const scoreTag = document.getElementById('tetris-score');
  scoreTag.textContent = score;
}

const renderLoop = () => {
  fps.render();
  draw();
  setScore();
  if (game.is_game_over()) {
    alert('game over !');
    return;
  }
  requestAnimationFrame(renderLoop);
};
renderLoop();

setInterval(() => {
  game.tick();
}, 300);

document.addEventListener('keydown', ({ key }) => {
  if (key === 'ArrowLeft' || key == 'a') {
    game.go_left();
  }
  if (key === 'ArrowRight' || key == 'd') {
    game.go_right();
  }
  if (key === 'ArrowDown' || key == 's') {
    game.go_bottom();
  }
  if (key == 'm') {
    toggleAudio();
  }
});

function getIndex(row, column) {
  return row * columnCount + column;
}

const url = 'https://ia600504.us.archive.org/33/items/TetrisThemeMusic/Tetris.mp3';
const audio = new Audio(url);
let isPlaying = false;

function toggleAudio() {
  if (isPlaying) {
    pause();
  } else {
    play();
  }
}

function play() {
  audio.addEventListener('ended', function () {
    if (isPlaying) {
      audio.play();
    }
  });
  audio.play().then(() => {
    isPlaying = true;
  });
}

function pause() {
  isPlaying = false;
  audio.pause();
}

play();
