// 初期化処理
document.addEventListener('DOMContentLoaded', function() {
    let input = document.getElementById('input_x');
    input.value = 0;
}, {'once': true});

function func() {
    window.__TAURI__
      .invoke("my_custom_command")
      .then((response) => {
      })
      .catch((error) => {
      })
  }

function on_value_changed() {
    let input = document.getElementById('input_x');
  window.__TAURI__
    .invoke("on_value_changed", { value: parseFloat(input.value) })
    .then((response) => {
    })
    .catch((error) => {
    })
}

// バックエンドからのメッセージのハンドリング
window.__TAURI__.event.listen('back-to-front', event => {
  console.log(`back-to-front ${event.payload} ${new Date()}`)
});
