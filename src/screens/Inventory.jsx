import React from 'react'
import NavBar from './components/navBar'

export default function InventoryScreen() {
  return (
    <div class="bg-[#FAFBFF] w-full h-screen flex items-center justify-center flex-col">
      <NavBar />
      <div
        class="w-full pt-5 flex flex-col items-center overflow-y-scroll"
        style={{ height: "90%" }}
      >
        <div class="text-[#ee1c25] text-3xl font-mono font-bold mb-5 ">
          Inventory
        </div>
      </div>
    </div>
  )
}
