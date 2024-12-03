# Final Project: Building a Multithreaded Web Server

For the final project, we'll make a web server that says "hello" and, in a web browser, looks like:

![](https://rust-book.cs.brown.edu/img/trpl20-01.png)

Here's the plan for building it:

1. Learn a bit about TCP and HTTP.
2. Listen for TCP connections on a socket.
3. Parse a small number of HTTP requests.
4. Create a proper HTTP response.
5. Improve the throughput of our server with a thread pool.
