# node-t2sdk-rs

This example demonstrates how to directly call T2SDK dynamic library from Rust and wrap it for NodeJS. However, please note that this is just a sample implementation and there are several important considerations:

- There is performance overhead when converting between Rust and JavaScript objects. This example only provides a basic implementation. The correct approach should involve wrapping higher-level APIs to avoid frequent object conversions.
- In fact, T2's packet format is extremely simple and common. In some simple scenarios, it's possible to implement the functionality without relying on the dynamic library.

## Project Status

**Feasibility Study/Experimental**: This project is currently in the feasibility exploration phase. It serves as an experimental implementation to evaluate the approach and is not ready for production use.


## Document
https://ufx.hs.net/#/index?ct=d81cf7dcf76b49e6aeb01bf830200976


## License

MIT License with Attribution Requirement

Copyright (c) 2025 William Chan

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

1. The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
2. Attribution to the original author must be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.