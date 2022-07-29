
import {useParams} from "react-router-dom"
import {useState, useEffect} from "react"
import styles from './ChatRoomPage.module.css'
import axios from "axios";


const {location} = window
const proto = location.protocol.startsWith("https") ? "wss" : "ws"

function ChatRoomPage() {
    let {name, uuid} = useParams()

    const wsUrl = `${proto}://127.0.0.1:8080/api/${uuid}`

    const [webSocket, setWebSocket] = useState(null);
    const [messageText, setMessageText] = useState("")
    const [receivedMessages, setMessages] = useState([])


    useEffect( () => {
        console.log("call")
        if(!webSocket) {
            setWebSocket(new WebSocket(wsUrl))
            return
        }
        if(!webSocket.onmessage) {
            webSocket.onmessage = e => {
                    console.log(e)
                    //const history = receivedMessages.slice(-199)
                    //console.log(receivedMessages)
                    setMessages(history => [...(history.slice(-199)), e])
                } 
            console.log("on message event listener added to socket")
        }
        if(!webSocket.onclose) {
            webSocket.onclose = e => {
                console.log("Web socket disconnected")
            }
        }
        /* previous solution was to add and remove event listener on every render 
        return ( () => {
            console.log("on message event listener removed from socket")
            webSocket.onmessage = null
        })*/
    }, [webSocket, wsUrl])


    let inputBox = null
    let messageEnd = null

    
    let session = null; // TODO: verify user login

    const messageTextIsEmpty = messageText.trim().length === 0
    const sendChatMessage = (messageText) => {
        if (webSocket) {
            webSocket.send(messageText)
        }

        setMessageText("")
        inputBox.focus()
    }

    const handleFormSubmission = (event) => {
        event.preventDefault();
        //if (!session) {
        //  alert("Please sign in first in order to chat")
        //  return
        //}
        sendChatMessage(messageText);
      }

    const handleKeyPress = (event) => {
        if (event.charCode !== 13 || messageTextIsEmpty) {
          return
        }
        //if (!session) {
         // alert("Please sign in first in order to chat")
         // return
        //}
        sendChatMessage(messageText)
        event.preventDefault()
    }

    const messages = receivedMessages.map((originalMessage, index) => {
        const author = session ? session.user.name.split(" ").at(0) : 'anonymous'
        const message = `${author}: ${originalMessage.data}`

        return <div key={index} className={styles.message} data-author={author}>{message} </div>
      })

      useEffect(() => {
        messageEnd.scrollIntoView({ behaviour: "smooth" });
      })


    return (<> 
        <h1> Welcome to {name}!</h1>
        <div className={styles.chatHolder}>
          <div className={styles.chatText}>
            {messages}
            <div ref={(element) => { messageEnd = element; }}></div>
          </div>
          <form onSubmit={handleFormSubmission} className={styles.form}>
            <textarea
              ref={(element) => { inputBox = element; }}
              value={messageText}
              placeholder="Type a message..."
              onChange={e => setMessageText(e.target.value)}
              onKeyPress={handleKeyPress}
              className={styles.textarea}
            ></textarea>
            <button type="submit" className={styles.button} disabled={messageTextIsEmpty}>Send</button>
          </form>
        </div>
    </>)

}

export default ChatRoomPage