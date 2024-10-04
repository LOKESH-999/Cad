import { useEffect, useRef, useState } from "react";
import "./drop.css"
import { addListener } from "@reduxjs/toolkit";
import { hide } from "@tauri-apps/api/app";

const SearchableDropdown = ({
  options,
  label,
  id,
  selectedVal,
  handleChange,
  handleSearch,
  invi,
  plus,
  type = 1,
  index,
  placeholder
}) => {
  const [query, setQuery] = useState(selectedVal);
  const [isOpen, setIsOpen] = useState(false);

  const inputRef = useRef(null);

  useEffect(() => {
    document.addEventListener("click", toggle);
    return () => document.removeEventListener("click", toggle);
  }, []);

  const selectOption = (option) => {
    setQuery(() => "");
    if (index != undefined){
    handleChange(index,option[label]);
    }
    else{
      handleChange(option[label])
    }
    setIsOpen((isOpen) => !isOpen);
  };

  function toggle(e) {
    setIsOpen(e && e.target === inputRef.current);
  }

  const getDisplayValue = () => {
    if (query) return query;
    if (selectedVal) return selectedVal;

    return "";
  };

  const filter = (options) => {
    const temp = options.filter(
      (option) => option[label].toLowerCase().indexOf(query.toLowerCase()) > -1
    );
    // handleSearch(temp)
    return temp
  };

  return (
    <div className="dropdown border-2" style={plus ? {height:"100%"}:{}}>
      <div className="control w-full h-full" >
        <div className="selected-value w-full h-full" >
          <input
            ref={inputRef}
            type={type == 1 ? "text" : "numeric"}
            value={getDisplayValue()}
            name="searchTerm"
            class="w-full h-full border-2 outline-blue-200 text-center text-black text-lg font-mono font-bold placeholder:text-black active:border-0 active:border-red-500 active:ring-1 active:ring-red-500"
            placeholder={placeholder}
            onChange={(e) => {
              setQuery(e.target.value);
              handleSearch(e.target.value)
              handleChange(null);
            }}
            onClick={toggle}
            autoComplete="off"
            
          />
        </div>
        {type == 1 &&  <div className={`arrow ${isOpen ? "open" : ""} self-center`}></div>}
       
      </div>

      {!invi && <div className={`options ${isOpen ? "open" : ""}`}>
        {filter(options).map((option, index) => {
          return (
            <div
              onClick={() => selectOption(option)}
              className={`option ${
                option[label] === selectedVal ? "selected" : ""
              }`}
              key={`${id}-${index}`}
            >
              {option[label]}
            </div>
          );
        })}
      </div>}
    </div>
  );
};

export default SearchableDropdown;
