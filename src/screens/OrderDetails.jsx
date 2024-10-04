import React,{useState,useEffect} from "react";
import NavBar from "./components/navBar";
import { useLocation, useNavigate} from "react-router-dom";
import { invoke } from "@tauri-apps/api";

export default function OrderDetailsScreen({routes}) {
  const navigation = useNavigate()
  const location = useLocation();
  const { order_id } = location.state || {};
  console.log(order_id)
    const heading = [
        "description",
        "brand",
        "cases/jib on pillets",
        "no of pallets",
        "Total No of JIBS/Bottles",
        "Price"
      ];
      const [data,setData] = useState([])
      const [metaData,setMeta] = useState({})
      const handlePacking = () => {
        navigation("/PackingList",{ state: { meta: metaData,data:data } })

      }
    useEffect(() => {
      const getData = async() => {
        // console.log(order_id,200)
        const temp = await invoke("get_orders_by_id",{orderId:order_id,custId:0})
        console.log(temp[0])
        const tempy = await invoke("get_order_lists_by_id",{id:order_id})
        console.log(tempy)
        let total = 0
        for (let i = 0;i < tempy.length;i++){
          // console.log(tempy[i]["bottles"]*tempy[i]["cases"])
          total += tempy[i]["bottles"] * tempy[i]["cases"]
        }
        // console.log(total,300)
        temp[0]["total"] = total
        setMeta(temp[0])
        setData(tempy)
      }
      getData()
    },[])
  return (
    <div class="bg-[#FAFBFF] w-full h-screen flex items-center justify-center flex-col">
      <NavBar />
      <div
        class="flex w-full flex-col pt-1 items-center"
        style={{ height: "70%" }}
      >
        <div class="flex flex-col items-center justify-start p-10 w-full overflow-y-scroll">
        <div class="text-black text-3xl font-mono font-bold mb-5 ">
          Kalaiselvan
        </div>
        <div class="flex flex-row w-full justify-between">
            <div class="flex flex-row w-3/12 gap-3 items-center">
            <div class="text-xl font-bold font-mono text-black">
              Total Price : 
            </div>
            <div class="text-xl font-bold font-mono text-black">
              {metaData.amount} 
            </div>
            </div>
            <div class="flex flex-row mr-5 w-3/12 gap-3 justify-end items-center">
            <div class="text-xl font-bold font-mono text-black">
              Total Weight : 
            </div>
            <div class="text-xl font-bold font-mono text-black">
              {metaData.n_weight} KG
            </div>
            </div>
          </div>
          <table class="min-w-full divide-y divide-gray-200 my-5">
            <thead class="bg-gray-50 border-2 border-[#ee1c25]">
              <tr>
                {heading.map((item) => {
                  return (
                    <th class="px-6 py-3 text-center text-md font-bold font-mono text-white uppercase tracking-wider bg-[#ee1c25] border border-white ">
                      {item}
                    </th>
                  );
                })}
              </tr>
            </thead>
            <tbody>
              {data.map((item) => {
                item["total"] = item["bottles"] * item["cases"]
                return (
                  <tr class="border-2 border-white bg-slate-200">
                    {["oil","brand","cases","bottles","total","cost"].map((itemy) => {
                      return (
                        <td class="px-6 py-4 whitespace-nowrap text-md font-bold font-mono text-black border-2 border-white">
                        {item[itemy]}
                        </td>
                      );
                    })}
                  </tr>
                );
              })}
               <tr class="border-2 border-white bg-slate-200">
                    {["","","Line Total",metaData.bottles,metaData.total,""].map((itemy) => {
                      return (
                        <td class="px-6 py-4 whitespace-nowrap text-md font-bold font-mono text-black border-2 border-white">
                        {itemy}
                        </td>
                      );
                    })}
                  </tr>

            </tbody>
          </table>
        </div>
      </div>
      <div class="flex gap-5 w-full items-center justify-center" style={{ height: "20%" }}>
      <button class="border-black border-dashed border bg-slate-200 font-mono font-bold text-black rounded" style={{height:50,width:"17%"}} onClick={handlePacking}>PRINT PACKINGLIST</button>
      <button class="border-black border-dashed border bg-slate-200 font-mono font-bold text-black rounded" style={{height:50,width:"17%"}}>PRINT INVOICE</button>
      <button class="border-black border-dashed border bg-slate-200 font-mono font-bold text-black rounded" style={{height:50,width:"17%"}}>MAIL INVOICE</button>
      <button class="border-black border-dashed border bg-slate-200 font-mono font-bold text-black rounded" style={{height:50,width:"17%"}}>SEND PAYMENT REMINDER</button>
      <button class="border-black border-dashed border bg-slate-200 font-mono font-bold text-black rounded" style={{height:50,width:"17%"}}>DELETE ORDER</button>     
      </div>
    </div>
  );
}
