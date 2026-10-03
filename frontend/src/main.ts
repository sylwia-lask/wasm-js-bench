import { mount } from 'svelte'
import './app.css'
import App from './App.svelte'

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    const base = import.meta.env.BASE_URL
    navigator.serviceWorker
      .register(`${base}sw.js`, { scope: base })
      .catch(() => {
        // Registration failures are non-fatal; the app runs without the SW.
      })
  })
}

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app
