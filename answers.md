Title: Problem Set 1 Answers
Author: Devin Lee jy4ny

1. Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/29.0.1547.62 Safari/537.36

  This describes the chrome browser version 29.0.1547.62, which is mozilla 5.0 compatible. AppleWebKit describes the set of things that are used to describe the content, Linux describes the OS of the client.

2. This is because of the race condition hazard, where the result of some other process may depend on the timing of other events that may possibly be concurrent. So Rust will notice the imminent possibility of error and warns ahead. Even though it will not really affect much in our case, it is still risky.