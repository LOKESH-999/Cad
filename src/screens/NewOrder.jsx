import React, { useEffect, useState } from "react";
import NavBar from "./components/navBar";
import { useNavigate } from "react-router-dom";
import SearchableDropdown from "./components/SearchableDropDown";
import { invoke } from "@tauri-apps/api";
// import { Provider } from "react-redux";
import SearchablePlus from "./components/SearchablePlus";

export default function NewOrderScreen() {
  
  const navigation = useNavigate();
  const [CustNames, setCustNames] = useState([]);
  const [products, setProducts] = useState([]);
  const [brand,setBrand] = useState([])
  const [name, setValue] = useState("");
  const [terms,setTerms] = useState("");
  const [price,setPrice] = useState(0);
  const [weight,setWeight] = useState(0);
  const [message,setMessage] = useState("")
  const heading = [
    ["Description", "35%", "md"],
    ["Brand", "35%", "md"],
    ["Cases/jib on pillets", "10%", "sm"],
    ["No of pallets", "10%", "sm"],
    ["Total No of Bottles", "10%", "sm"],
    ["Price","10%","sm"]
  ];
  const type = [1, 1, 2, 2, 2];
  const [fields, setFields] = useState(["", "", 0, 0, 0, 0]);
  const [data, setData] = useState([]);

  const handleDataChange = (index, event, mode) => {
    console.log(event,2000)
    const newFields = [...fields];
    if (index == 0){
      handleDescPrice(event)
      return
    }
    newFields[index] = event;
    if (newFields[2] && newFields[3]){
      newFields[4] = newFields[2] * newFields[3]
    }
    console.log(newFields, 101);
    if (
      index == 4 &&
      mode != "typing" &&
      newFields.every((i) => i != "" || i != 0)
    ) {
      handleAdd(newFields);
      return;
    }
    
    setFields(newFields);
  };
  const handleEnter = (event) => {
    if (event.key === 'Enter') {
      console.log(99)
                event.preventDefault();
                handleDataChange(4,Number(event.target.value))
            }
}
  const handleAdd = (newy) => {
    let temp = [];
    if (newy) {
      temp = newy;
    } else {
      temp = fields;
    }
    const base = products.filter(item => item.descs == temp[0])[0]
    setPrice(price + (temp[5] * temp[4]))
    setWeight(weight + (temp[4] * base.bottel_in_pallet))
    temp = [...data,temp]
    setData(temp);
    setFields(["", "", 0, 0, 0, 0]);
  };
  const handleAddOrder = async () => {
    const temp = CustNames.filter((item) => item.name == name)[0];
    let pallets = 0;
    let jibs = 0;
    for(let i = 0 ; i < data.length;i++){
      pallets +=  data[i][3]
      jibs += data[i][2]
    }
    const value = {
       cust_id:temp.cust_id,
       m_batches: false,
       amount: price,
       pending_amount: price-1,
       order_list:data.map((item) => {
        return {
           oil: item[0],
           brand: item[1],
           cases:  item[3],
           bottles:  item[2],
           cost: 10,
           n_weights: 10,
        };
      }),
       batch_data:null,
       msg: message,
       n_weights: weight,
       cases: pallets,
       bottles: jibs,
    }
   console.log(value)
    await invoke("place_order", {
      dueDate:terms,
      data:value,
    });
    navigation("/Orders");
  };
  const handleDescPrice = (desc) => {
    const temp = [...fields]
    console.log(products,202)
    const target = products.filter(item => item.descs == desc)[0].price
    console.log(target,"daddasdas")
    temp[0] = desc
    temp[5] = target
    setFields(temp)
  }
  useEffect(() => {
    const getData = async () => {
      const temp = await invoke("get_package");
      console.log(temp)
      const tempy = await invoke("get_all_customer");
      let tempo = await invoke("get_brand")
      tempo = tempo.map(item => {return {brand:item} })
      console.log(tempo,200)
      setProducts(temp);
      setCustNames(tempy);
      setBrand(tempo)
    };
    getData();
  },[]);
  return (
    <div class="bg-[#FAFBFF] w-full h-screen flex items-center justify-center flex-col">
      <NavBar />
      <div
        class="flex w-full flex-col pt-5 items-center"
        style={{ height: "90%" }}
      >
        <div class="flex-col h-full items-center justify-start p-10 w-full overflow-y-scroll">
          <div class="flex flex-row">
            <input
              class="w-3/12 text-center border-2 border-slate-200 mr-2 font-mono font-bold text-lg text-black focus:outline-0"
              type="text"
              placeholder="Enter terms of days"
              onChange={(e) => setTerms(Number(e.target.value))}
            />
            <SearchableDropdown
              options={CustNames}
              label="name"
              id="id"
              selectedVal={name}
              handleChange={(val) => setValue(val)}
              placeholder={"Customers"}
            />
            <div class="ml-2 flex flex-row w-3/12 justify-between items-center">
            <div class="text-md font-bold font-mono text-black">
              Total Price : 
            </div>
            <input
               placeholder="0"
               value={price}
               class="w-7/12 border-2 h-full text-center font-mono font-bold placeholder:text-red"
               type = "number"
               disabled={true}
              
            />
            </div>
            <div class="ml-2 flex flex-row w-3/12 justify-between items-center">
            <div class="text-md font-bold font-mono text-black">
              Total Weight : 
            </div>
            <input
               placeholder="0"
               value={weight}
               class="w-7/12 border-2 h-full text-center font-mono font-bold placeholder:text-red"
               type = "number"
               disabled={true}
              
            />
            </div>
          </div>

          <table class="min-w-full w-full divide-y divide-gray-200 my-5">
            <thead class="bg-gray-50 border-2 border-[#FFFFFF]">
              <tr>
                {heading.map((item) => {
                  return (
                    <th
                      style={{ width: item[1] }}
                      class={`text-center text-${item[2]} font-bold font-mono text-white bg-[#ee1c25] tracking-wider border-2 border-white`}
                    >
                      {item[0]}
                    </th>
                  );
                })}
              </tr>
            </thead>
            <tbody>
              {data.map((item) => {
                return (
                  <tr class="border-2 border-white bg-slate-200">
                    {item.map((itemy, index) => {
                      return (
                        <td
                          style={{ width: heading[index][1] }}
                          class="px-6 py-4 whitespace-nowrap text-md font-bold font-mono text-black border-2 border-white"
                        >
                          {itemy}
                        </td>
                      );
                    })}
                  </tr>
                );
              })}
            </tbody>
          </table>
          <div
            class="w-full flex flex-row bg-black mt-10 mb-3"
            style={{ height: 60 }}
          >
            {[[products,"descs"], [brand,"brand"], [[],""], [[],""],].map((item, index) => {
              return (
                <div class="flex" style={{ width: heading[index][1] }}>
                  <SearchablePlus
                    options={item[0]}
                    label={item[1]}
                    id="id"
                    selectedVal={fields[index]}
                    handleChange={handleDataChange}
                    index={index}
                    type={type[index]}
                    placeholder={heading[index][0]}
                    freq={[1, 2, 3]}
                  />
                </div>
              );
            })}
            <div class="flex" style={{width:"10%"}} >
            <input
               placeholder="0"
               value={fields[4]}
               class="w-full border-2 text-slate-200 h-full text-center font-mono font-bold placeholder:text-black"
               type = "number"
               disabled={true}
              
            />
            </div>
            <div class="flex" style={{width:"5%"}} >
            <input
               placeholder="0"
               value={fields[5]}
               class="w-full border-2  h-full text-center font-mono font-bold placeholder:text-black"
               type = "number"
               onChange={(e) => handleDataChange(5,Number(e.target.value))}
               onKeyDown={handleEnter}
              
            />
            </div>
          </div>
          <textarea
          class="w-full mt-8 font-mono font-bold text-lg text-black border-2 focus:outline-0"
          style={{
            height:100
          }}
          onChange={(e) => setMessage(e.target.value)}
          type="text"
          placeholder="Enter customer message"
          
        />
          <div class="flex mt-5 items-center justify-center">
            <button
              class="w-3/5 font-mono text-xl font-bold text-[#ee1c25] uppercase rounded-md"
              style={{ height: 40 }}
              onClick={handleAddOrder}
            >
              PLACE ORDER
            </button>
          </div>
        </div>
      </div>
      {/* <div
        class="flex w-full border-t-2 border-black justify-center items-center"
        style={{ height: "20%" }}
      >
        <select class="ml-2 bg-slate-200 flex flex-1 items-center justify-center border-2 border-dashed border-black text-center text-black text-lg font-mono font-bold" style={{height:50}} name="cars" id="cars" onChange={(e) => handleDropy(e)}>
              {products.map(item => {return(
            <option class="bg-[#ee1c25] text-white text-center font-mono font-bold" value={item.value}>{item.name}</option>
              )})}
          </select>
        {["Brand", "Cases on pillets", "No of pillets", "Total"].map((placeholder, index) => (
          <div key={index} className="flex flex-1 items-center justify-center">
            <input
              className="w-3/4 bg-slate-200 border-2 border-black border-dashed text-center text-black text-lg font-mono font-bold placeholder:text-black active:border-0 active:border-blue-500 active:ring-1 active:ring-blue-500"
              style={{ height: 50 }}
              type="text"
              placeholder={placeholder}
              value={fields[index+1]}
              onChange={(e) => handleDataChange(index+1, e)}
            />
          </div>
        ))}
        <div class="flex flex-1 items-center justify-center">
        <button class="w-3/5 font-mono text-xl font-bold text-[#ee1c25] uppercase rounded-md" style={{height:40}} onClick={handleAdd}>ADD +</button>
        </div>

        </div>  */}
    </div>
  );
}
