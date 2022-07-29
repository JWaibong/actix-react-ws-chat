import {useState, useEffect} from "react"
import axios from "axios"
import React from "react"

import {
    Link
  } from "react-router-dom";

function RoomListElement({uuid, name, ...props}) {




    return (<li> <Link to={`/chat/${name}/${uuid}`}> {name} </Link> </li>)
    

}

export default RoomListElement;