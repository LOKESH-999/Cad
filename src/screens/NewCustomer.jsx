import React, { useState } from 'react'
import NavBar from './components/navBar'
import user from "../assets/user1.svg"
import email from "../assets/mail.svg"
import hst from "../assets/hst.svg"
import phone from "../assets/phone.svg"
import map from "../assets/map.svg"
import company from "../assets/company.svg"
import { useNavigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api'

export default function NewCustomerScreen() {
    const navigation = useNavigate()
    const [name,setName] = useState('')
    const [Phone,setPhone] = useState('')
    const [addr,setAddr] = useState('')
    const [Email,setEmail] = useState('')
    const [Hst,setHst] = useState('')
    const [Company,setCompany] = useState('')

    const handleAddCustomer = async() => {
        await invoke("create_customer",{userName:name,company:Company,phone:Number(Phone),hst:Hst,email:Email,address:addr})
        navigation("/Customers")
    }
  return (
    <div class="bg-[#FAFBFF] w-full h-screen flex items-center justify-center flex-col">
      <NavBar />
      <div class="flex flex-col  justify-center w-full" style={{ height: "90%" }}>
        <div class="w-full flex items-center justify-center gap-10" style={{height:"10%"}}> 
        <div class="w-4/12 h-full flex  mt-5 mb-5 text-[#ee1c25] border-2 pfont-mono font-bold text-lg rounded-md">
        <div class="w-3/12 bg-white flex items-center justify-center">
        <img src={user} alt="" style={{height:"40%"}}/>
        </div>
        <input
          class="w-9/12 font-mono font-bold text-lg text-black focus:outline-0"
          type="text"
          placeholder="Enter customer name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        </div>
        <div class="w-4/12 h-full flex mt-5 mb-5 text-[#ee1c25] border-2 pfont-mono font-bold text-lg rounded-md">
        <div class="w-3/12 bg-white flex items-center justify-center">
        <img src={email} alt="" style={{height:"40%"}}/>
        </div>
        <input
          class="w-9/12 font-mono font-bold text-lg text-black focus:outline-0"
          type="email"
          placeholder="Enter customer email"
          value={Email}
          onChange={(e) => setEmail(e.target.value)}
        />
        </div>      
        </div>
        <div class="w-full flex items-center justify-center my-4" style={{height:"10%"}}> 
        <div class="w-4/12 h-full flex mt-5 mb-5 text-[#ee1c25] border-2 pfont-mono font-bold text-lg rounded-md">
        <div class="w-3/12 bg-white flex items-center justify-center">
        <img src={hst} alt="" style={{height:"40%"}}/>
        </div>
        <input
          class="w-9/12 font-mono font-bold text-lg text-black focus:outline-0"
          type="text"
          placeholder="Enter customer HST"
          value={Hst}
          onChange={(e) => setHst(e.target.value)}
        />
        </div>      
        </div>
        <div class="w-full flex items-center justify-center gap-10" style={{height:"10%"}}> 
        <div class="w-4/12 h-full flex  mt-5 mb-5 text-[#ee1c25] border-2 pfont-mono font-bold text-lg rounded-md">
        <div class="w-3/12 bg-white flex items-center justify-center">
        <img src={company} alt="" style={{height:"40%"}}/>
        </div>
        <input
          class="w-9/12 font-mono font-bold text-lg text-black focus:outline-0"
          type="text"
          placeholder="Enter customer company name"
          value={Company}
          onChange={(e) => setCompany(e.target.value)}
        />
        </div>
        <div class="w-4/12 h-full flex mt-5 mb-5 text-[#ee1c25] border-2 pfont-mono font-bold text-lg rounded-md">
        <div class="w-3/12 bg-white flex items-center justify-center">
        <img src={phone} alt="" style={{height:"40%"}}/>
        </div>
        <input
          class="w-9/12 font-mono font-bold text-lg text-black focus:outline-0"
          type="text"
          placeholder="Enter customer phone"
          value={Phone}
          onChange={(e) => setPhone(e.target.value)}
        />
        </div>      
        </div>
        <div class="w-full flex items-center justify-center my-4" style={{height:"30%"}}> 
        <div class="w-4/12 h-full flex mt-5 mb-5 text-[#ee1c25] border-2 pfont-mono font-bold text-lg rounded-md">
        <div class="w-3/12 bg-white flex items-center justify-center">
        <img src={map} alt="" style={{height:"40%"}}/>
        </div>
        <textarea
          class="w-9/12 font-mono font-bold text-lg text-black border-2 focus:outline-0"
          type="text"
          placeholder="Enter customer address"
          value={addr}
          onChange={(e) => setAddr(e.target.value)}
        />
        </div>      
        </div>
        <div
        class="flex w-full items-center justify-center"
        style={{ height: "10%" }}
      >
        <button class="w-1/5 font-mono text-xl font-bold text-[#ee1c25] uppercase rounded-md" onClick={handleAddCustomer}>Add customer</button>
      </div>
      </div>
    </div>
  )
}
