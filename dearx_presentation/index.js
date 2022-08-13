function func() {
    window.__TAURI__
      .invoke("my_custom_command")
      .then((response) => {
      })
      .catch((error) => {
      })
  }

  function on_value_changed() {

    window.__TAURI__
      .invoke("on_value_changed", { value: parseFloat(input.value) })
      .then((response) => {
      })
      .catch((error) => {
      })
  }

  let input = document.getElementById('input_x');
  input.value = 0;

  window.__TAURI__.event.listen('back-to-front', event => {
    console.log(`back-to-front ${event.payload} ${new Date()}`)
  });