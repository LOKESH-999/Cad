import React from "react";
import { NavLink } from "react-router-dom";

export default function NavBar() {
  return (
    <div
      class="flex white justify-between items-center px-10"
      style={{ height: "10%", width: "100%" }}
    >
      <div class="w-1/2 items-center justify-center text-black font-mono font-bold text-xl">
        Canaddin Pride
      </div>
      <NavLink to="/Home">
        <div class="text-black  font-mono font-bold">
          Home
        </div>
      </NavLink>
      <NavLink to="/Orders">
        <div class="text-black font-mono font-bold ">
          Orders
        </div>
      </NavLink>
      <NavLink to="/Customers">
        <div class="text-black font-mono font-bold">
          Customers
        </div>
      </NavLink>
      <NavLink to="/Reminder">
        <div class="text-black font-mono font-bold">
          Reminder
        </div>
      </NavLink>
      <NavLink to="/Inventory">
        <div class="text-black font-mono font-bold">
          Inventory
        </div>
      </NavLink>
      <NavLink to="/">
        <div class="uppercase rounded-xl text-[#ee1c25] text-md font-bold">
          Logout
        </div>
      </NavLink>
    </div>
  );
}
