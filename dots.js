

//SETUP

//wasm_bindgen.foo(arr);
function show_dots(ctx2, dots) {
const dots_new = dots.map((element) => Object.fromEntries(element))
const data = {
  datasets: [{
    label: 'Random dots',
    data: dots_new,
    backgroundColor: function(context) {
      const index = context.dataIndex;
      const value = context.dataset.data[index];
      return value["x"]*value["x"] + value["y"]*value["y"] <= 1.0 ? 'blue' :  // in-circle is blue
        'green' ;    // else, green
    }
  }],
};
//CONFIG
const config = {
  type: 'scatter',
  data: data,
  options: {
    aspectRatio: 1,
    scales: {
      x: {
        type: 'linear',
        min: -1.0,
        max: 1.0,
      },
      y: {
        type: 'linear',
        min: -1.0,
        max: 1.0,
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