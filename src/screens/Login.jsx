import logo from "../assets/Capture.svg";
import { useNavigate } from "react-router-dom";
import user from "../assets/user1.svg"
import pass from "../assets/pass.svg"
import { invoke } from '@tauri-apps/api';
import { useState } from "react";

function LoginScreen() {
  const navigation = useNavigate();
  const [username,setUsername] = useState("")
  const [passwd,setPassword] = useState("")

  const handleNavigation = async() => {
    // login logic
    // await invoke("login",{username,passwd})
    navigation("/Home");
  };
  return (
    <div class="w-full h-screen flex items-center justify-center flex-col">
      <img src={logo} alt="" />
      <div class="rounded-xl w-3/5 h-3/5 items-center flex flex-col mt-5">
        <h1 class="text-5xl text-[#ee1c25] font-bold font-mono">Login</h1>
        <div class="w-1/2 flex mt-5 mb-5 text-[#ee1c25] border-2 pfont-mono font-bold text-lg rounded-md" style={{height:"15%"}}>
        <div class="w-3/12 flex items-center justify-center">
        <img src={user} alt="" style={{height:"40%"}}/>
        </div>
        <input
          class="w-9/12 font-mono font-bold text-lg text-black focus:outline-0"
          type="text"
          placeholder="Enter your username"
          onChange={(e) => setUsername(e.target.value)}
        />
        </div>
        
        <div class="w-1/2 flex mt-5 mb-5 text-[#ee1c25] border-2  rounded-md" style={{height:"15%"}}>
        <div class="w-3/12 flex items-center justify-center">
        <img src={pass} alt="" style={{height:"40%"}}/>
        </div>
        <input
          class="w-9/12 font-mono font-bold text-lg text-black focus:outline-0"
          type="password"
          placeholder="Enter your password"
          onChange={(e) => setPassword(e.target.value)}
        />
        </div>
        <button
          class="w-2/5 rounded text-white text-lg mt-5 bg-[#ee1c25] font-mono font-bold"
          style={{ height: "12%" }}
          onClick={handleNavigation}
        >
          login {">>>"}
        </button>
      </div>
    </div>
  );
}

export default LoginScreen;
