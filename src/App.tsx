import React from "react";
import Sidebar from "@components/Sidebar/Sidebar";
import Map from "@components/Map";

const App = () => {
  return (
    <div className="flex flex-row">
      <Sidebar />
      <div className="h-screen flex-1 bg-gray-100">
        <Map />
      </div>
    </div>
  );
};

export default App;
