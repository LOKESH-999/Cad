import React from "react";
import NavBar from "./components/navBar";
import CanvasJSReact from "@canvasjs/react-charts";

export default function HomeScreen() {
  var CanvasJSChart = CanvasJSReact.CanvasJSChart;
  const options = {
    backgroundColor:"white",
    animationEnabled: true,
    exportEnabled: true,
    theme: "light2", //"light1", "dark1", "dark2"
    title: {
      fontColor:"red",
      text: "Revenue",
    },
    axisY: {
      includeZero: true,
      lineColor: "red",
      gridColor:"red"
    },
    data: [
      {
        color:"black",
        type: "column", //change type to bar, line, area, pie, etc
        //indexLabel: "{y}", //Shows y value on all Data Points
        indexLabelFontColor: "white",
        indexLabelPlacement: "outside",
        dataPoints: [
          { x: 10, y: 71 },
          { x: 20, y: 55 },
          { x: 30, y: 50 },
          { x: 40, y: 65 },
          { x: 50, y: 71 },
          { x: 60, y: 68 },
        ],
      },
    ],
  };
  return (
    <div class="bg-[#FAFBFF] w-full h-screen flex items-center justify-center flex-col">
      <NavBar />
      <div class="w-full bg-black pt-5" style={{ height: "90%" }}>
        <div class="flex flex-row">
          <div class="w-1/2">
            <CanvasJSChart options={options} />
          </div>
          <div class="w-1/2">
            <CanvasJSChart options={options} />
          </div>
        </div>
      </div>
    </div>
  );
}
