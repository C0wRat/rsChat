
<p align="center">
  <img src="/imgs/logo.png" alt="rsChat Logo" width="200" />
</p>


# rsChat

**rsChat** is a lightweight, browser-based chat application written in Rust, designed to allow multiple users to communicate in real time. It supports hosting or joining chat rooms with a simple yet functional user interface. With plans for future enhancements, rsChat is the perfect base for developers looking to expand on a Rust-based chat client.


<p align="center">
  <img src="/imgs/RSchat.png" alt="rsChat Logo" width="200" />
</p>


## Features

### 1. **Real-Time Messaging**
rsChat offers real-time messaging where multiple users can connect to a single server. Messages are broadcasted instantly to all users in the chat room, ensuring smooth communication.

### 2. **Host or Join Chat Rooms**
Users can either host their own chat rooms or join an existing session. This flexibility allows for quick and easy communication, whether you're setting up a group chat or joining a pre-existing conversation.

### 3. **Custom Usernames**
Personalize your chat experience by setting a custom username upon joining. This feature allows for easy identification of participants within the chat room.

### 4. **Browser-Based Interface**
The chat client is fully accessible through a modern web browser, eliminating the need for users to install additional software. The intuitive graphical user interface (GUI) provides a seamless experience on both desktop and mobile browsers.

## Planned Features

While rsChat is functional, there are several planned enhancements in the works:

### 1. **End-to-End Encryption**
To ensure privacy and data security, future updates will include end-to-end encryption. This feature will allow messages to be encrypted on the sender’s side and decrypted only by the recipient, keeping the communication private from intermediaries.

### 2. **Message Persistence**
Implement message history, allowing users to review past messages when they reconnect to the chat room.

### 3. **User Authentication**
Integrate user authentication to ensure that only authorized participants can join specific chat rooms, adding a layer of security.


## Installation and Usage

### Prerequisites
- Ensure that you have Rust installed on your machine. If not, you can follow the [official installation guide](https://www.rust-lang.org/tools/install).
- A modern web browser (Chrome, Firefox, Edge, etc.)

### Steps to Run:

1. Clone this repository:
    ```bash
    git clone https://github.com/your-repo/rsChat.git
    ```
   
2. Navigate to the project directory:
    ```bash
    cd rsChat
    ```
   
3. Run the application with Cargo:
    ```bash
    cargo run
    ```

4. Open your web browser and navigate to **[rschat.rat-trap.io](http://rschat.rat-trap.io/)**. You can either host a new chat room or join an existing one.