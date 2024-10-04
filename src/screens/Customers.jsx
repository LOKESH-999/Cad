import React,{useState,useEffect} from 'react'
import { useNavigate } from 'react-router-dom';
import NavBar from './components/navBar'
import SearchableDropdown from './components/SearchableDropDown';
import user from "../assets/user.svg"
import { invoke } from '@tauri-apps/api';

export default function CustomersScreen() {
    const navigation = useNavigate()
    const [CustNames,setCustomer] = useState([])
    const [sCustNames,setSCustNames] = useState(CustNames)
    const [value, setValue] = useState("");
      const renderItem = (item) => {
        return (
          
            <button class="flex flex-row h-32 bg-slate-200 rounded-xl">
                <div class="flex w-3/12 h-full items-center justify-center">
                <img src={user} alt="" style={{height:"40%"}}/>
                </div>
                <div class="flex w-9/12 h-full flex-col justify-center items-start">
                <div class="text-black text-xl uppercase font-sans font-semibold">{item.name}</div>
                <div class="text-black text-lg font-mono font-semibold">{item.id}</div>
                </div>
            </button>
        )
      }
    const handleNavigation = () => {
      navigation("/NewCustomer")
    }
    const handleSearch = (query) => {
        const temp =  CustNames.filter(
            (option) => option["name"].toLowerCase().indexOf(query.toLowerCase()) > -1
          );
        setSCustNames(temp)
    }
    useEffect(() => {
    const getData = async() => {
      const temp = await invoke("get_all_customers",{})
      setCustomer(temp)
      setSCustNames(temp)
    }
    getData()
    },[])
  return (
    <div class="bg-[#FAFBFF] w-full h-screen flex items-center justify-center flex-col">
      <NavBar />
      <div
        class="w-full pt-5 flex flex-col items-center overflow-y-scroll"
        style={{ height: "80%" }}
      >
        <SearchableDropdown
                options={CustNames}
                label="name"
                id="id"
                selectedVal={value}
                handleChange={(val) => setValue(val)}
                handleSearch={handleSearch}
                invi={true}
              />
        <div class="w-4/5 mt-5">
        <div class="grid grid-cols-3 gap-4">
            {sCustNames.map(item => renderItem(item))}
        </div>
        </div>
        
      </div>
      <div
        class="flex w-full border-2 items-center justify-center"
        style={{ height: "10%" }}
      >
        <button class="w-1/5 font-mono text-xl font-bold text-[#ee1c25] uppercase rounded-md" onClick={handleNavigation}>New customer</button>
      </div>
    </div>
  )
}
