# cluesolver
Application to take information about a Clue/Cluedo game in progress and make deductions

This is the source for the [clue solver](https://gregstoll.com/cluesolver/) site.

- [`cluesolver/public/index.html`](https://github.com/gregstoll/cluesolver/blob/master/cluesolver/public/index.html) is the main page 
- [`cluesolver/src/App.tsx`](https://github.com/gregstoll/cluesolver/blob/master/cluesolver/src/App.tsx) is the TypeScript source for the app
- [`clueengine_rust/src/lib.rs`](https://github.com/gregstoll/cluesolver/blob/master/clueengine_rust/src/lib.rs) is the Rust library that has all of the logic
- [`clueengine_rust/src/bin/cgi_server.rs`](https://github.com/gregstoll/cluesolver/blob/master/clueengine_rust/src/bin/cgi_server.rs) is the Rust CGI script that the UI calls into

For the previous version written in Python:
- [`clueengine.py`](https://github.com/gregstoll/cluesolver/blob/master/clueengine.py) has all of the logic. (and unit tests)
- [`clue.py`](https://github.com/gregstoll/cluesolver/blob/master/clue.py) is the CGI script that the UI calls into
