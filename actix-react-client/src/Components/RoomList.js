import {useState, useEffect} from "react"
import axios from "axios"
import React from "react"
import RoomListElement from "./RoomListElement"

function RoomList() {
    let [data, setData] = useState(null)
    useEffect( () => {
      console.log("Use Effect called in Room List")
      axios.get('/api/current_rooms')
        .then( res => {
          console.log(res)
          setData(res.data)
        })
    }, [])

    let lst = !data ? <> </> : data.map( ele => {
      let [uuid, name] = ele.split(" ")
      
      return  <RoomListElement uuid={uuid} name={name} key={uuid}/>
    })

    return (
      <> {!data ? "Loading" : <ul> {lst} </ul>} </>
    )

}
export default RoomList