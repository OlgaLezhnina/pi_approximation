

//SETUP

//wasm_bindgen.foo(arr);
function chart_pi(ctx, arr, labels) {
const data = {
  labels: labels,
  datasets: [
    {
      label: 'Dataset 1',
      data: arr,
      
    }    
  ]
};
//CONFIG
const config = {
  type: 'line',
  data: data,
  options: {
    responsive: true,
    plugins: {
      legend: {
        position: 'top',
      },
      title: {
        display: true,
        text: 'Chart.js Line Chart'
      }
    }
  },
};
//CHART
new Chart(ctx, config);
}