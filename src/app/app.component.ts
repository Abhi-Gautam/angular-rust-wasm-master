import {Component, ElementRef, ViewChild} from '@angular/core';
import { Chart } from 'chart.js';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent  {
  @ViewChild('canvas') canvas: ElementRef;
  executionTime11 = 0;
  executionTime12 = 0;

  executionTime21 = 0;

  matrixSize = 1000;
  plotReady: boolean = false;
  public chart: any;
  distribution = [];
  min: number;
  max: number;
  binSize: number;
  resultMatrix = Array.from({ length: this.matrixSize }, () => new Array(this.matrixSize).fill(0));
  resultMatrixFlattened: any[];
  wasmModule: any;

  ngOnInit(): void {
    const init = async () => {
      this.wasmModule = await import('../../wasm/pkg/rust_webpack_template');
      this.wasmModule.main_js();
      this.wasmModule.draw();
      const result = await this.wasmModule.get_image_diff_array("https://i.imgur.com/5YuMlQC.jpeg", "https://i.imgur.com/5YuMlQC.jpeg");
      console.log(result);
    };
    init();
  }

  runMatrxiMultiplicationOnRust(): void {
    const startTime = performance.now();
    const resultMatrix = this.wasmModule.matrix_multiply(this.matrixSize);
    const endTime = performance.now();
    this.executionTime21 = endTime - startTime;
  }

  runMatrixMultiplication(): void {
    const size = this.matrixSize;
    let matrixA = Array.from({ length: size }, () => Array.from({ length: size }, () => Math.random()));
    let matrixB = Array.from({ length: size }, () => Array.from({ length: size }, () => Math.random()));

    const startTime = performance.now();

    // Matrix multiplication
    for (let i = 0; i < size; i++) {
      for (let j = 0; j < size; j++) {
        for (let k = 0; k < size; k++) {
          this.resultMatrix[i][j] += matrixA[i][k] * matrixB[k][j];
        }
      }
    }

    this.resultMatrixFlattened = this.resultMatrix.flat();
    const endTime = performance.now();
    this.executionTime11 = endTime - startTime;
  }

  analyzeDistribution(): void {
    this.executionTime12 = 0;
    const startTime = performance.now();
    const flatResults = this.resultMatrixFlattened;
    this.max = Math.max(...flatResults);
    this.min = Math.min(...flatResults);
    const binCount = 10;
    this.binSize = (this.max - this.min) / binCount;
    this.distribution = Array(binCount).fill(0);

    for (let value of flatResults) {
      const index = Math.floor((value - this.min) / this.binSize);
      if (index === binCount) { // Handle the max value case
        this.distribution[binCount - 1]++;
      } else {
        this.distribution[index]++;
      }
    }
    this.plotDistribution();
    const endTime = performance.now();
    this.executionTime12 = endTime - startTime;
  }

  plotDistribution(): void {
    const labels = Array.from({ length: this.distribution.length }, (_, i) => (this.min + i * this.binSize).toFixed(2));
    const data = {
      labels: labels,
      datasets: [{
        label: 'Value Distribution',
        backgroundColor: 'rgba(75, 192, 192, 0.2)',
        borderColor: 'rgba(75, 192, 192, 1)',
        data: this.distribution,
      }]
    };

    new Chart(this.canvas.nativeElement.getContext('2d'), {
      type: 'bar',
      data: data,
      options: {
        scales: {
          yAxes: [{
            ticks: {
              beginAtZero: true
            }
          }]
        }
      }
    });
  }
}
