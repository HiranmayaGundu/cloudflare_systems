Hi! I'm Hiranmaya Gundu, a Master's student at University of Southern California.

While I have been learning Rust for a while, it was mostly by solving some leetcode problems using Rust.
This is my first *relatively* serious Rust project! This is the first time I have built a web server in Rust.

This is the first time I have had to work "hands on" with JWT too! While my previous org used it, all the code 
to sign in and include it in the Authorization header was already written, so I never needed to interact with that
portion of the code.

For me, the most difficult part was (rather surprisingly) finding a way to work with Cookies. There did not seem to be a
straightforward way to add a cookie to a NamedFile response, which is the suggested way of serving files in actix: https://actix.rs/docs/static-files/

I ended up diving into the actix-files codebase and finding this line to help me get the response: https://github.com/actix/actix-web/blob/c020cedb631d08528d42de24741975b0a5b6e0b6/actix-files/src/service.rs#L126
and from there was able to figure the rest out. 

The other problem I ran into was that that jsonwebtoken supports **unencrypted** private RSA pems, while the commands generate an encrypted pem by default.
I found out that was the issue by comparing the pem that I had with the one in the test cases for the jsonwebtoken repo here: https://github.com/Keats/jsonwebtoken/tree/master/tests/rsa
I saw the extra Proc-Type: 4,ENCRYPTED header which made me realize I needed an unencrypted pem!

I have implemented both the bonus endpoints. The stats endpoint responds with a json object containing the number of times the /verify 
and /auth endpoints are hit, as well as the time the encode /decode calls takes in milliseconds.

