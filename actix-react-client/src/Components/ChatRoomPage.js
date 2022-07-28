
import {useParams} from "react-router-dom"
import {useState, useEffect} from "react"
import styles from './ChatRoomPage.module.css'
import axios from "axios";
function ChatRoomPage() {
    let {name, uuid} = useParams()

    let [webSocket, setWebSocket] = useState(null);
    let inputBox = null
    let messageEnd = null

    useEffect( () => {
        const {location} = window
        const proto = location.protocol.startsWith("https") ? "wss" : "ws"
        const wsUri = `${proto}://127.0.0.1:8080/api/${uuid}`
        //setWebSocket(new WebSocket(`ws://http://127.0.0.1:8080/api/${uuid}`))
        const ws = new WebSocket(wsUri)
        setWebSocket(ws)
        console.log("Successfuly established web socket")
    }, [uuid])
    
    let session = null; // TODO: verify user login

    const [messageText, setMessageText] = useState("")
    const [receivedMessages, setMessages] = useState([])

    const messageTextIsEmpty = messageText.trim().length === 0

//    const [channel, ably] = useChannel("chat-demo", message => {
//          const history = receivedMessages.slice(-199)
//            setMessages([...history, message])
//    }) 
// TODO: enable message sending

    const sendChatMessage = (messageText) => {
        // channel.publish({ name: "chat-message", data: messageText });
        
        setMessageText("");
        inputBox.focus();
    }

    const handleFormSubmission = (event) => {
        event.preventDefault();
        if (!session) {
          alert("Please sign in first in order to chat")
          return
        }
        sendChatMessage(messageText);
      }

    const handleKeyPress = (event) => {
        if (event.charCode !== 13 || messageTextIsEmpty) {
          return
        }
        if (!session) {
          alert("Please sign in first in order to chat")
          return
        }
        sendChatMessage(messageText);
        event.preventDefault();
    }

    const messages = receivedMessages.map((originalMessage, index) => {
        const author = session ? session.user.name.split(" ").at(0) : 'anonymous'
        const message = `${author}: ${originalMessage.data}`

        return <div key={index} className={styles.message} data-author={author}>{message} </div>
      });

      useEffect(() => {
        messageEnd.scrollIntoView({ behaviour: "smooth" });
      });


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