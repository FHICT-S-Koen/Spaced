import './index.css';

import Vec2D from "./lib/vector"

const app = document.getElementById('app') as HTMLElement

const textarea = document.createElement('textarea')

textarea.classList.add('absolute', 'border')

app.appendChild(textarea)

let lastMousePosition = new Vec2D(0, 0)
let pos = new Vec2D(0, 0)

app.addEventListener('mousemove', (event) => {
  console.log(event.buttons)
  if (event.buttons === 1) {
    pos = pos.add(new Vec2D(event.clientX-lastMousePosition.x, event.clientY-lastMousePosition.y))
    textarea.style.transform = `translate(${pos.x}px, ${pos.y}px)`
  }
  lastMousePosition = new Vec2D(event.clientX, event.clientY)
})
