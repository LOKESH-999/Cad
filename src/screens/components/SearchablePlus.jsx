import React,{useState} from "react";
import SearchableDropdown from "./SearchableDropDown";

export default function SearchablePlus({
  options,
  label,
  id,
  selectedVal,
  handleChange,
  handleSearch,
  invi,
  type,
  freq,
  index,
  placeholder
}) {
    // document.getElementById("4").addEventListener('keydown', function(event) {
    //     if (event.key === 'Enter') {
    //         event.preventDefault();
    //         console.log(111)
    //         // handleChange(4,Number(event.target.value),"typing")
    //     }
    // })
    const handleEnter = (event) => {
        if (event.key === 'Enter') {
                    event.preventDefault();
                    handleChange(4,Number(event.target.value))
                }
    }
  return (
    <div class="flex  w-full">
      <div class="w-7/12" style={{height:"100%"}}>
      {type == 1 ? <SearchableDropdown
          options={options}
          label={label}
          id={id}
          selectedVal={selectedVal}
          handleChange={handleChange}
          handleSearch={handleSearch}
          invi={invi}
          plus={true}
          type={type}
          index={index}
          placeholder={placeholder}
        />:
        <input
        value={selectedVal}
        id={index}
        onChange={(e) => {
            handleChange(index,Number(e.target.value),"typing")}}
        onKeyDown={index == 4 ? handleEnter : function a () {}} 
        placeholder="0"
        class="w-full h-full border-2  text-center font-mono font-bold placeholder:text-black"
        type = "number"

        />}
        
      </div>
      <div class="flex w-5/12 h-full flex-col">
        {freq.map(item => 
        <button class="flex-1 bg-slate-200 border-b border-white" onClick={() => handleChange(index,item)}>{item}</button>
        )}
      </div>
    </div>
  );
}
