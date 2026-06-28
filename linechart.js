
function chart_pi(ctx, arr, labels) {
  //SETUP
  const data = {
    labels: labels,
    datasets: [
      {
        data: arr,
        borderWidth: 1,
        pointRadius: 0,
        pointHoverRadius: 2,
        borderColor: '#000',
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
          display: false,
        },
        title: {
          display: true,
          text: 'Approximated pi value'
        }
      }
    },
  };
  //CHART
  new Chart(ctx, config);
}