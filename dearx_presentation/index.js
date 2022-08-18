// import { listen, emit } from '@tauri-apps/api/event'

// 初期化処理
document.addEventListener('DOMContentLoaded', function () {
  let input = document.getElementById('input_x');
  input.value = 0;
}, { 'once': true });

function func() {
  window.__TAURI__
    .invoke("my_custom_command")
    .then((response) => {
    })
    .catch((error) => {
    });

  //バックエンドにメッセージを投げる
  window.__TAURI__.event.emit('front-to-back', { message: 'Tauri is awesome!' });
}

function on_value_changed() {
  let input = document.getElementById('input_x');

  //バックエンドにメッセージを投げる
  window.__TAURI__.event.emit('input_x_changed', { value: parseFloat(input.value) });
}

function on_selection_changed(id) {
  //バックエンドにメッセージを投げる
  window.__TAURI__.event.emit('selection_changed', { id: id });
}

// バックエンドからのメッセージのハンドリング
window.__TAURI__.event.listen('back-to-front', event => {
  let input = document.getElementById('button');
  input.textContent = `${new Date()}`;
});

