// App.js
import "./App.css"
import { Routes, Route } from 'react-router-dom';
import LoginScreen from './screens/Login';
import HomeScreen from './screens/Home';
import OrdersScreen from './screens/Orders';
import NewOrderScreen from './screens/NewOrder';
import OrderDetailsScreen from './screens/OrderDetails';
import ReminderScreen from './screens/Reminder';
import InventoryScreen from "./screens/Inventory";
import CustomersScreen from "./screens/Customers";
import NewCustomerScreen from "./screens/NewCustomer";
import PackingList from "./screens/PackingList";


 
const App = () => {
   return (
      <>
         <Routes>
            <Route path="/" element={<LoginScreen />} />
            <Route path="/Home" element={<HomeScreen />} />
            <Route path="/Orders" element={<OrdersScreen />} />
            <Route path="/NewOrders" element={<NewOrderScreen />} />
            <Route path="/OrderDetails" element={<OrderDetailsScreen/>} />
            <Route path="/Reminder" element={<ReminderScreen />} />
            <Route path="/Inventory" element={<InventoryScreen />} />
            <Route path="/Customers" element={<CustomersScreen />} />
            <Route path="/NewCustomer" element={<NewCustomerScreen />} />
            <Route path="/PackingList" element={<PackingList />} />






            {/* <Route path="/products" element={<Products />} />
            <Route path="/about" element={<About />} /> */}
         </Routes>
      </>
   );
};
 
export default App;