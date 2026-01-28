let chartInstance = null;

window.createOrUpdateChart = function(canvasId, labels, buyData, rentData) {
    const canvas = document.getElementById(canvasId);
    if (!canvas) return;

    const ctx = canvas.getContext('2d');

    if (chartInstance) {
        chartInstance.data.labels = labels;
        chartInstance.data.datasets[0].data = buyData;
        chartInstance.data.datasets[1].data = rentData;
        chartInstance.update('none');
        return;
    }

    chartInstance = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: 'Buy (Net Worth)',
                    data: buyData,
                    borderColor: '#2563eb',
                    backgroundColor: 'rgba(37, 99, 235, 0.1)',
                    fill: true,
                    tension: 0.3,
                    pointRadius: 4,
                    pointHoverRadius: 6,
                },
                {
                    label: 'Rent (Investments)',
                    data: rentData,
                    borderColor: '#dc2626',
                    backgroundColor: 'rgba(220, 38, 38, 0.1)',
                    fill: true,
                    tension: 0.3,
                    pointRadius: 4,
                    pointHoverRadius: 6,
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            interaction: {
                intersect: false,
                mode: 'index',
            },
            plugins: {
                legend: {
                    position: 'bottom',
                    labels: {
                        usePointStyle: true,
                        padding: 20,
                    }
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            let value = context.parsed.y;
                            return context.dataset.label + ': $' + value.toLocaleString('en-US', {maximumFractionDigits: 0});
                        }
                    }
                }
            },
            scales: {
                x: {
                    title: {
                        display: true,
                        text: 'Years'
                    },
                    grid: {
                        display: false
                    }
                },
                y: {
                    title: {
                        display: true,
                        text: 'Net Worth ($)'
                    },
                    ticks: {
                        callback: function(value) {
                            if (Math.abs(value) >= 1000000) {
                                return '$' + (value / 1000000).toFixed(1) + 'M';
                            } else if (Math.abs(value) >= 1000) {
                                return '$' + (value / 1000).toFixed(0) + 'K';
                            }
                            return '$' + value;
                        }
                    }
                }
            }
        }
    });
};
