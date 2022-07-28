import logo from './logo.svg';
import './App.css';
import {useState, useEffect} from "react"
import axios from "axios"
import Home from "./Components/Home"
import NavBar from './Components/NavBar';
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Link
} from "react-router-dom";
import ChatRoomPage from './Components/ChatRoomPage';

function App() {

  return (
    <Router>

      <div className="App">
        <NavBar>
        
        </NavBar>
        <Routes>
          <Route path="/" element={<Home/>}/>

          <Route path="/chat/:name/:uuid" element={<ChatRoomPage />}/>
        </Routes>
      </div>
    </Router>
  );
}

export default App;
