# Rust messaging example

This project is an example of how to use Rust programming language to create Pokedex and Pokemon game app. For database PostgreSQL is used.


### Project structure

The project is structured as:

bins - bins directory, contains code for binaries that will be built and executed, ex: WebServer
docs - contains hand written OpenAPI documentation (ts)
lib - contains libraries for the project, these are separated from the bins as they will (should) contain logic that can be used throughout the project.
migrations - diesel migrations directory, use Diesel CLI to manage the migrations
support - support files, test files, email templates, etc. In general, mostly static resources required by the application
The root Cargo.toml file defines the project's workspace. Adding new binaries or libraries must be reflected inside of that file.

### Coding guide

This is a short coding guide that revolves around the philosophy of contracts, based on the Ports and Adapters Pattern (More info on this in the Sources section below), 
explained on the example of auth inside of the web server (monolith).

We are observing the authentication directory and the explanation of files is as follows:

##### data.rs
holds the data structure definitions, these will be struct(s) that hold the data from the incoming request, used to parse and validate the data.

This file can also be made as a directory data containing mod.rs and all of the required structs in separate files, ex. struct for handling login data is inside login_request.rs, struct for handling register data is inside 
register_request.rs and so on, and then linked inside of the data dir's mod.rs with pub mod <file>. This logic applies to all of the files inside of this dir, 
but is mostly used when the files containing the data related logic are too large and contain a lot of structs; for readability.

#### http.rs (or http dir, as described above)
contains functions that handle (validate) incoming request data and pass it on to the service level structs. These functions also regulate requests based on the api version provided in the request (ex /api/v1 or api/v2, etc.)

#### contract.rs - contracts file
contains contracts for our services. 

Contracts are traits that define functions that the structs that implement them must have. Think of it as this: A contract contains a set of rules that must be satisfied if another struct implements the contract.

####infrastructure.rs
This file contains the implementation details based on the defined contract(s).
These are mostly named Repository or Service. You can look at Repositories as structs that get the data from the database and Services as structs that manipulate the data (create, update, delete). For example: 
If we have a simple Auth domain that needs to get users from the db, create users and send an email when the user is registered, we would have something in the lines of:

#### PostgresRepository - for getting auth related data from the db
#### PostgresService - for storing auth related data to the db
#### EmailService - for management of emails on user auth actions

#### domain.rs
basically the main file of our module contains the main struct (module) that is created with all of the defined services, modules, etc. based on the contracts defined.

#### setup.rs
setup the endpoints in this file. It should contain one function called routes that sets up the routes/endpoints for the current module.

#### mod.rs
mod file, includes all of the files/directories inside the module directory(in this case, authentication.)

#### test.rs
tests file; if tests are too large, can be refactored into a dir (as described above)




#### Error handling

Errors are handled throughout the application in a central place, located inside the `libs/error` library.

For more info see:

- [Error handling README](./libs/error/README.md#error-handling)
  - [How to add errors](./libs/error/README.md#how-to-add-errors)
  - [Error Index](./libs/error/README.md#error-index)

## Credits

This app was created by Matej Zagar.
