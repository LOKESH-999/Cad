import React, { useEffect } from "react";
import { useLocation } from "react-router-dom";
import "./Packing.css";
import minc from "../assets/minc.svg";
import logo from "../assets/Capture.svg";
import bar from "../assets/bar.svg";

export default function PackingList() {
  const location = useLocation();
  const { meta, data } = location.state || {};
  const date1 = new Date(meta.order_date.slice(0,10));

  const months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];

  const monthName = months[date1.getMonth()];
  const day = date1.getDate();
  const year = date1.getFullYear();
  const date2 = new Date(meta.term.slice(0,10));
  const monthName1 = months[date2.getMonth()];
  const day2 = date2.getDate();
  const year2 = date2.getFullYear();

  const Datey = `${monthName} ${day}, ${year}`;
  const Datex = `${monthName1} ${day2}, ${year2}`;

  function subtractDates(date1Str, date2Str) {
    const date1 = new Date(date1Str);
    const date2 = new Date(date2Str);
    
    const diffInMs = date1 - date2;
    
    const diffInDays = diffInMs / (24 * 60 * 60 * 1000);
    
    return diffInDays;
}
  const topdata = [
    ["Packing List No 90-06/", meta.order_id],
    ["Date", Datey],
    ["Customer Code", meta.cust_id],
    ["Terms(days)", subtractDates(meta.order_date.slice(0,10),meta.term.slice(0,10)) * -1],
  ];
  const sizes = ["35%", "15%", "20%", "15%", "25.4%"];
  const renderRow = (item) => {
    return (
      <div
        class="flex flex-row border-b-2 border-l-2  border-black"
        style={{ width: "100%", height: "10%" }}
      >
        {["oil","brand","cases","bottles","total"].map((itemy, index) => (
          <div
            class={`text-center h-full border-r-2 border-black font-bold`}
            style={{ width: sizes[index], fontSize: index == 0 ? 13 : 16 }}
          >
            {item[itemy]}
          </div>
        ))}
      </div>
    );
  };
  useEffect(() => {
    // Add the class to the body when this component is mounted
    document.body.classList.add("body");

    // Cleanup function to remove the class when the component is unmounted
    return () => {
      document.body.classList.remove("body");
    };
  }, []);
  return (
    <div>
      <div
        class="border-black flex items-end justify-between"
        style={{
          marginLeft: 10,
          width: "270mm",
          height: "20.7mm",
          borderBottomWidth: 3,
        }}
      >
        <div class="text-3xl font-bold mb-1">Canaddin Pride foods Inc.</div>
        <div class="text-3xl font-bold yellowy bg-[#ffff00] pb-3 px-3 pt-1">
          Packing List
        </div>
      </div>
      <div
        class="flex items-end justify-between"
        style={{ marginLeft: 10, width: "270mm", height: "30.7mm" }}
      >
        <div class="flex flex-col h-full w-6/12 p-2 ">
          <div class="flex-1 border-t-2 border-r-2 border-l-2 border-black">
            {topdata.map((item) => (
              <div
                class="flex-1 flex flex-row border-b-2 border-black "
                style={{ width: "100%", height: "25%" }}
              >
                <div class="w-8/12 h-full text-sm font-bold text-end pr-1">
                  {item[0]}
                </div>
                <div class="w-4/12  border-l-2 h-full text-xs pl-1 font-bold border-black">
                  {item[1]}
                </div>
              </div>
            ))}
          </div>
          <div class="text-sm font-black pl-20 ml-10">
            Payment due {Datex}
          </div>
        </div>
        <div class="flex flex-row h-full w-6/12">
          <div class="flex-5 w-4/12  items-center flex">
            <img src={minc} alt="" />
          </div>
          <div class="flex-7 ">
            <img src={logo} alt="" />
          </div>
        </div>
      </div>
      <div
        class="flex items-end justify-between"
        style={{ marginLeft: 10, width: "270mm", height: "45.7mm" }}
      >
        <div class="flex-1 flex flex-col  h-full">
          <div style={{ height: "60%" }}>
            <div class="font-bold">Kalaiselvan</div>
            {[
              "1420172 Ontario Ltd",
              "O/A Dairy Max 5050 Edwards Blvd",
              "Mississauga",
            ].map((row) => (
              <div>{row}</div>
            ))}
          </div>
          <div class="flex p-1" style={{ height: "50%" }}>
            <div class="flex-1 border-2 border-black">
              <div
                class="flex-1 border-b-2 border-black bg-slate-200"
                style={{ width: "100%", height: "33%" }}
              ></div>
              <div
                class="flex-1 font-bold text-sm border-b-2 border-black"
                style={{ width: "100%", height: "34%" }}
              >
                Customer po
              </div>
              <div
                class="flex-1 font-bold text-sm"
                style={{ width: "100%", height: "33%" }}
              >
                See Customer Message
              </div>
            </div>
          </div>
        </div>
        <div class="flex-1 items-center flex flex-col justify-center h-full">
          <div class="text-[#806000] font-bold text-lg text-center mb-10">
            Truly Canadian Edible oils
          </div>
          <img src={bar} alt="" class="mt-5" />
        </div>
      </div>
      <div
        style={{
          marginTop: 10,
          marginLeft: 10,
          width: "270mm",
          height: "80.7mm",
        }}
      >
        <div style={{ width: "100%", height: "90%" }}>
          <div
            class="border-2 border-black flex-row flex"
            style={{ height: "15%", width: "100%" }}
          >
            <div
              class="text-center h-full border-r-2 border-black font-bold"
              style={{ width: "35%" }}
            >
              Description
            </div>
            <div
              class="text-center h-full border-r-2 border-black font-bold"
              style={{ width: "15%" }}
            >
              Brand
            </div>
            <div
              class="text-center h-full border-r-2 border-black font-bold"
              style={{ width: "20%", fontSize: 14.8 }}
            >
              Cases/JIBs on Pallet
            </div>
            <div
              class="text-center h-full border-r-2 border-black font-bold"
              style={{ width: "15%" }}
            >
              No of Pallets
            </div>
            <div
              class="text-center h-full font-bold"
              style={{ width: "25%", fontSize: 16 }}
            >
              Total No of JIBS/Bottles
            </div>
          </div>
          {data.map((item, index) => renderRow(item))}
          <div
            class="flex flex-row border-b-2 border-l-2  border-black"
            style={{ width: "100%", height: "10%" }}
          >
            {[
              ["70%", "Line Total"],
              ["15%", meta.bottles],
              ["25.2%", meta.total],
            ].map((item, index) => (
              <div
                class={`text-center h-full border-r-2 border-black font-bold pr-2`}
                style={{
                  width: item[0],
                  textAlign: index == 0 ? "end" : "center",
                }}
              >
                {item[1]}
              </div>
            ))}
          </div>
          <div
            class="text-sm font-black"
            style={{ marginLeft: 205, width: "100%" }}
          >
            <span>Gross weight of the load</span>
            <span class="ml-10">{meta.n_weight} Kgs</span>
          </div>
        </div>
      </div>
      <div
        style={{
          marginTop: 10,
          marginLeft: 10,
          width: "270mm",
          height: "70.7mm",
        }}
      >
        <div
          class="flex-row flex justify-end items-end"
          style={{ width: "100%", height: "15%" }}
        >
          <div
            class="bg-[#ffff00] text-end font-semibold pr-4"
            style={{ paddingLeft: "30%" }}
          >
            Shipper Shipped by seller
          </div>
        </div>
        <div
          class="border-black"
          style={{ width: "100%", height: "85%", borderWidth: 3 }}
        >
          <div class="font-bold text-sm">Customer Message:</div>
          <div class="p-1 text-md" style={{ width: "100%", height: "65%" }}>
            {meta.msg}
          </div>
          <div class="text-xs text-[#ff0100]">
            are on returnable basis. The same may be held at the buyers end till
            goods held on them are sold. The seller be informed when ready , and
            pick up will be arranged by seller, else @ $15.00 will be chargeable
            per pallet. PALLETS ARE BLUE IN COLOR
          </div>
        </div>
        <div
          class="bg-slate-200 border-2 border-black"
          style={{ width: "100%", height: "8%" }}
        ></div>
        <div
          class="flex h-full border-black"
          style={{ width: "100%", height: "25%", borderWidth: 3 }}
        >
          <div style={{ width: "33%", height: "100%" }}>
            <div class="text-sm">GST/HST No</div>
            <div class="text-sm">Tel No</div>
            <div class="text-sm">Email</div>
          </div>
          <div style={{ width: "33%", height: "100%" }}>
            <div class="text-sm">HST No: 794688325RT0001</div>
            <div class="text-sm">905 228 6506</div>
            <div class="text-sm">ravi@canaddinpridefoods.com</div>
          </div>
          <div style={{ width: "34%", height: "100%" }}>
            <div class="text-sm">
              <span class="font-bold">Company name </span>
              <span>Canaddin Pride Foods Inc</span>
            </div>
            <div class="text-sm">
              <span class="font-bold">Address</span>
              <span style={{ marginLeft: 58 }}>2220 Argentia Rd # 9</span>
            </div>
            <div class="text-sm">
              <span class="font-bold"></span>
              <span style={{ marginLeft: 110 }}>Mississauga, ON, L5N2K7</span>
            </div>
          </div>
        </div>
      </div>
      <div
        style={{
          marginLeft: 10,
          marginTop: 110,
          width: "270mm",
          height: "20.7mm",
        }}
      >
        <div class="text-[#806000] font-bold text-center">
          GLOBAL REACH CANADIAN TOUCH
        </div>
        <div class="border-2 border-black mb-1"></div>
        <div class="border-2 border-black"></div>
      </div>
    </div>
  );
}
