# Ray Tracing in a Weekend in Rust

I previously did this with Go but felt like it could go much faster which gave me an excuse to do it in rust and get the feel for concurrency in it.

This rust implementation is arround 3 times faster than my go implementation my Macbook Air.

I used rayon to add some concurrency and it was as easy than it is with goroutines and channels (rayon worked very nicely for this use case but goroutines and channels feel a lot more flexible).
