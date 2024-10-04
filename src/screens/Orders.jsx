import React,{useState,useEffect} from "react";
import NavBar from "./components/navBar";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api";

export default function OrdersScreen() {
  const [data,setData] = useState([])
  const height = window.screen.height;
  const navigation = useNavigate()
  const handleOrder = (id) =>{
    navigation("/OrderDetails",{ state: { order_id: id } })
  }
  const renderItem = (datay) => {
    datay["order_date"] = datay["order_date"].slice(0,10)
    datay["name"] = "Kalaiselvan"
    datay["pending_amount"] = datay["amount"]

    return (
      <button
        class="flex justify-between items-center mb-2 w-4/5 bg-slate-200 px-10 rounded-md"
        style={{ height: height * 0.1, marginLeft: "10%" }}
        onClick={() => handleOrder(datay["order_id"])}
      >
        {["order_id","name","order_date","amount","pending_amount","status"].map((item) => (
          <div class="flex flex-1 h-full items-center justify-center text-lg text-black font-sans font-semibold ">
            {item != "status" ? datay[item] : datay[item] == 1 && "Pending" }
          </div>
        ))}
      </button>
    );
  };
  const handleNavigation = () => {
    navigation("/NewOrders")
  }
  useEffect(() => {
    const getData  = async() => {
      const temp = await invoke("get_all_order")
      console.log(temp)
      setData(temp)
    }
    getData()
  },[])
  return (
    <div class="bg-[#FAFBFF] w-full h-screen flex items-center justify-center flex-col">
      <NavBar />
      <div class="w-full pt-5 overflow-y-scroll" style={{ height: "70%" }}>
        <div
          class="flex justify-between bg-[#ee1c25] rounded items-center mb-2 w-4/5 px-10"
          style={{ height: height * 0.07, marginLeft: "10%" }}
        >
          {[
            "ORDER ID",
            "NAME",
            "DATE",
            "AMOUNT",
            "PENDING AMOUNT",
            "STATUS",
          ].map((item) => (
            <div class="flex flex-1 h-full items-center justify-center text-center font-mono font-bold text-white">
              {item}
            </div>
          ))}
        </div>
        {data.map((item) => renderItem(item))}
      </div>
      <div
        class="flex border-t-2 w-full border-black justify-center items-center"
        style={{ height: "20%" }}
      >
        <div class="flex flex-1 items-center justify-center">
          <input
            class="w-3/4 border-2 border-slate-200 text-center text-black text-lg font-mono font-bold placeholder:text-black focus:outline-0"
            style={{ height: 50 }}
            type="text"
            placeholder="Search"
          />
        </div>
        <div class="flex flex-1 items-center justify-center gap-10 px-10">
            <div class="flex flex-1 flex-col justify-center items-center">
            <div class="flex flex-1 h-full items-center justify-center text-center font-mono font-bold text-black mb-2">
            STATUS
            </div>
          <select class="border-2 border-slate-200 px-2 text-black font-mono font-bold" name="cars" id="cars">
            <option class="bg-[#ee1c25] text-white font-mono font-bold" value="volvo">Paid</option>
            <option class="bg-[#ee1c25] text-white font-mono font-bold" value="saab">Pending</option>
            <option class="bg-[#ee1c25] text-white font-mono font-bold" value="mercedes">Partial</option>
          </select>
            </div>
            <div class="flex flex-1 flex-col justify-center items-center">
            <div class="flex flex-1 h-full items-center justify-center text-center font-mono font-bold text-black mb-2">
            DATE</div>
          <input class="border-2 border-slate-200 px-2 text-black font-mono font-bold" type="date" />
          </div>
          
          <button class="w-1/5 font-mono text-xl font-bold text-[#ee1c25] uppercase rounded-md" style={{height:40}} onClick={handleNavigation}>New Order</button>
        </div>
      </div>
    </div>
  );
}
