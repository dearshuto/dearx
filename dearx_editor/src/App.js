import logo from './logo.svg';
import './App.css';

import { invoke } from '@tauri-apps/api/tauri'
import { listen, emit } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog';

listen('back-to-front', event => {
  let input = document.getElementById('button');
  input.textContent = `${new Date()}`;
});


const App = () => {
  const func = () => {
    invoke("my_custom_command")
      .then((response) => {
      })
      .catch((error) => {
      });

    //バックエンドにメッセージを投げる
    emit('front-to-back', { message: 'Tauri is awesome!' });
  }

  const on_value_changed = () => {
    let input = document.getElementById('input_x');

    //バックエンドにメッセージを投げる
    emit('input_x_changed', { value: parseFloat(input.value) });
  }

  const on_selection_changed = (id) => {
    //バックエンドにメッセージを投げる
    emit('selection_changed', { id: id });
  }

  const open_dialog = () => {
    // Open a selection dialog for image files
    const selected = open({
      multiple: true,
      filters: [{
        name: 'file',
        extensions: ['*']
      }]
    });
    if (Array.isArray(selected)) {
      // user selected multiple files
    } else if (selected === null) {
      // user cancelled the selection
    } else {
      // user selected a single file
    }
  }

  return (
    <div className="a">
      <header className="header">
        <nav className="h-nav">
          <ul className="b">
            <li onClick={open_dialog}><a href="#">File</a></li>
            <li><a href="#">Edit</a></li>
            <li><a href="#">Tool</a></li>
            <li><a href="#">Window</a></li>
            <li><a href="#">Help</a></li>
          </ul>
        </nav>
      </header>
      <div className="c">
        <main className="main"><button id="button" onClick={func}>AB</button></main>
        <aside>
          <li>Root
            <ul>
              <li><button id="apple" onClick={() => on_selection_changed("appple")} >Apple</button></li>
              <li><button id="peach" onClick={() => on_selection_changed("peach")}>Peach</button></li>
            </ul>
          </li>
        </aside>
        <aside className="side">
          <p>X<input id="input_x" type="number" step="0.1" onChange={on_value_changed} /></p>
          <p>Y<input type="number" step="0.1" /></p>
          <p>Z<input type="number" step="0.1" /></p>
          <p>Color<input type="color" /></p>
        </aside>
      </div>
      <footer className="footer">
        dearx editor
      </footer>
    </div>
  );
}

export default App;
