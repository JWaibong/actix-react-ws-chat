import logo from './logo.svg';
import './App.css';
import {useState, useEffect} from "react"
import axios from "axios"
function App() {
  
  let [data, setData] = useState(null)

  useEffect( () => {
    axios.get('/api/current_rooms')
      .then( res => {
        console.log(res)
        setData(res.data)
      })
  }, [])

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          {!data ? "Loading" : data}
        </p>
      </header>
    </div>
  );
}

export default App;
