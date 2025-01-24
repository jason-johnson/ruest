# ruest
Crate to support writing a proper REST service in Rust.

# Router information

## Pre-work

The Ruest backend enables the use of REST content-types or "contracts" easily.  It also facilitates the automatic creation of clients by generating a schema that systems can use to generate views and routes for all content-types supported by a Ruest backend.  For a given client framework, there should be commandds which do the following:

* Take the URL to the Ruest backend
* Read /.schema.json from that backend
* Generate simple components based on the schema, which will contain all possible content-types the server supports
* Generate content-type based routes between these content types and the components which render them

## Clients

As mentioned above, Ruest clients are simple compared to modern clients in typical client/server architecture.  They do not need any logic beyond render logic because the logic is handled on the server side.  The client's only job is to send requests to the server and render the responses.  This means the only real components of a client are these components which render the "contracts" specified by the content type and the router which routes a given content-type to its registered component.

## Client and Router 

The way a client works in the Ruest is the following logic (web app, but native clients will be similar:

1) Client web application is opened at a URL (normally this would be /, at least on the first use.  But regardless of what the initial URL is, the behavior is the same)
2) The client app Router gets the request for the client side URL (e.g. /, /events, /pictures, etc.)
3) The router does a `GET` request to the configured server of the form of `GET {base URL}{client route}`.  NOTE: in each request the client makes, it must always send an `accept` header with all the content types (and their versions!) that it understands.  This allows the server to response with the best type the client is able to display.
4) The client gets back a response from the server
5) The router uses the `content-type` of the response to dispatch to the appropriate component
6) The body of the response is passed to the selected component for display
7) The component renders the data from the response as an HTML page

The component will almost certainyl have links and actions represented in the data sent by the server in the response.  The user of the client will click these links or buttons, which will generally cause either a new `GET` request (e.g. in the case of a link) or possibly some other HTTP method (e.g. `PUT`).

For the case of a `GET` this will generally be when the user clicks a "link".  The behavior in this case would be that clicking the link causes the link shown in the browser bar to change to the new one (e.g. if the user clicks a link which has an `a ref` pointing to `event/21.09.2025` then this would be in the browser bar and begin the "client" routing mechanism.  The client routing mechanism starts at step 2) in the logic description and follows the steps to completion.  This is the most common flow of this style of architecture.

For the case of other methods, it can be the case that some "contract" (described by the "content type") knows that certain fields in the data are meant to be rendered as a button which, when pressed, will do a `PUT`, `POST`, or `DELETE` message (or some other legal HTTP method).  This will be done with the standard `fetch` mechansms known in Svelte but the router should catch the request follow steps 2) through 7) above but using the correct method (e.g. `PUT`) instead of `GET`.  The relevant body must also be sent to the server.  But otherwise the logic is the same: after the e.g. `PUT` method is sent (and, again, this must include the `accept` header with all known content-types), the server will respond with a `content-type` and body to be routed the normal way.

## Example client

One example client to make use of such an architecture might be a bank application.  When the user opens the bank application ("https://client.mybank.com/" for this example, NOTE: we ignore authentication for this example), the client router send a request to the configured backend server ("https://rest.mybank.com/" for this example).  The server returns a response with a "root" component.  This component could be a dashboard type which shows all acounts of the user and, perhaps, common actions clients do in the bank application.  But in this instince, the client clicks on one of their accounts to view its screen.  The "root" component treats clicks on accounts like a "link", that is when the user clicks the account, the client route changes ("https://client.mybank.com/accounts/savings").  The client router then make the appropriate server request ("GET https://rest.mybank.com/accounts/savings").  At this point, the server applies business logic to determine exactly what is sent back.  For some accounts, users are allowed to withdraw money.  This depends on their rights on the account, the kind of account it is and how much money is in the account.  For this example the user is only working with accounts they have rights to and money can be withdrawn from if there are sufficient funds.  The server sends the standard "account" "content-type" back to the client but the schema for accounts says that there may or may not be a link for withdrawing money.  The client does not know or care why, it simply knows that if this link is there then it should render a button and field to facilitate the withdrawl and if not then these fields are shown "disabled" with a tool tip that says "insufficient funds".  In this case there are sufficient funds and the user wishes to perform a withdrawal, so they type in 100 and press the withdraw button.  The component is programmed such that this causes it to use the `fetch` API to do a `POST` to the correct location with the correct payload ("POST https://rest.mybank.com/accounts/savings/withdraw" with body "{ amount: 100 }").  The server handles the request and sends a response which is, again, routed by the client side router based on the content type.  NOTE: For all these activities, the client application always sends an `accept` header with all known content-types and the server always sends back the appropriate content-type if able or an error if not.
