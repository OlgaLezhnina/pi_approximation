function show_dots(ctx2, dots) {
  //SETUP
  const dots_new = dots.map((element) => Object.fromEntries(element))
  const data = {
    datasets: [{
      data: dots_new,
      backgroundColor: function(context) {
        const index = context.dataIndex;
        const value = context.dataset.data[index];
        return value["x"]*value["x"] + value["y"]*value["y"] <= 1.0 ? 'rgb(39, 190, 89)' : 'rgb(3, 56, 79)';
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
          display: false,
        },
        title: {
          display: true,
          text: 'Random dots (N = 10000) inside and outside the circle (r = 1)'
        }
      }
    },
  };
  //CHART
  new Chart(ctx2, config);
}