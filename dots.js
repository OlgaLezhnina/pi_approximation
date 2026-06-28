

//SETUP

//wasm_bindgen.foo(arr);
function show_dots(ctx2, dots) {
const data = {
  datasets: [{
    label: 'Approximation of pi',
    data: dots.entries_typed,
    backgroundColor: 'rgb(255, 99, 132)'
  }],
};
//CONFIG
const config = {
  type: 'scatter',
  data: data,
  options: {
    scales: {
      x: {
        type: 'linear',
        position: 'bottom'
      }
    },
    responsive: true,
    plugins: {
      legend: {
        position: 'top',
      },
      title: {
        display: true,
        text: 'Approximated pi value'
      }
    }
  },
};
//CHART
new Chart(ctx2, config);
}