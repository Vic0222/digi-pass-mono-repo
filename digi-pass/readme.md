###
This project is compose by multiple "domain slice". e.g. : Events Basket, etc.
Which I consider as a microservices in my mind. 
This has the following advantage
- By doing this it's easier for me to focus on one slice per goal. The same way I would if I was working on micrsoservices. 
- Since the main purpose of this project is for learning, everytime I want to try something new I can apply it to one slice and not worry that It may affect the other slices.


###
Each domain slice we try to follow a rule where only the following sub modules are public
- application module
    - which consist of application layer level services, which is the only way domain modules can interact with each other.
- data_transfer_objects
    - consist of models used to transport data between domain modules
- controller
    - the module the contains the handler used by Axum, the api framework we are using for this porject.

We try to follow the above rules so that when the time comes and we need to move to microservices we only need to change the application service implementation.