import React from "react";
import NavBar from "./components/navBar";
import read from "../assets/read.svg";
import unread from "../assets/unread.svg";
import time from "../assets/time.svg";

export default function ReminderScreen() {
  const data = [
    {
      status: "read",
      segment: "Payment",
      date: "19-08-2024",
      time: "7:00 AM",
      customer: "Madhur",
      message:
        "Order:171089 payment pending -> 3000 CAD order shipped on 24-07-2024 at 7:00 AM",
    },
    {
      status: "unread",
      segment: "Inventory",
      date: "17-08-2024",
      time: "9:00 PM",
      customer: "",
      message: "Canola oil needs restock",
    },
  ];
  const height = window.screen.height;
  const renderItem = (item) => {
    return (
      <div
        class="flex w-4/5 border-2 border-black bg-slate-200 mb-5 rounded-xl"
        style={{ height: height * 0.25 }}
      >
        <div class="flex w-3/12 items-center justify-center">
          <img
            src={item.status == "read" ? read : unread}
            style={{ height: "40%" }}
          />
        </div>
        <div class="flex flex-col w-6/12 items-center pt-2">
          <div
            class="bg-[#ee1c25] mb-2 font-mono font-bold text-white rounded-md flex items-center justify-center"
            style={{ height: 30, width: "30%" }}
          >
            {item.segment}
          </div>
          {item.customer && 
        <div class="text-black text-lg font-mono font-bold mb-1 ">
          {item.customer}  
          </div>}
        <div class="flex w-4/5 flex-grow items-center justify-center text-black text-lg font-mono font-bold mb-5 justify-self-center">
            {item.message}
        </div>
        </div>
        <div class="flex flex-col w-3/12">
          <div class="flex items-center justify-center gap-2 w-full h-2/5">
            <img src={time} style={{ height: "30%" }} />
            <div class="text-black font-mono font-bold">
              {item.date} at {item.time}
            </div>
          </div>
          <div class="flex w-full h-3/5 justify-center">
            {item.status == "unread" && (
              <button
                class="uppercase font-bold text-[#ee1c25] rounded-md text-md"
                style={{ height: "50%", width: "50%" }}
              >
                Mark as Read
              </button>
            )}
          </div>
        </div>
      </div>
    );
  };
  return (
    <div class="bg-[#FAFBFF] w-full h-screen flex items-center justify-center flex-col">
      <NavBar />
      <div
        class="w-full pt-5 flex flex-col items-center overflow-y-scroll"
        style={{ height: "90%" }}
      >
        {data.map((item) => renderItem(item))}
      </div>
    </div>
  );
}
