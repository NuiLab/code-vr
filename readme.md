![Cover Image][screenshot-1]

# Python VR

[![License][license-img]][license-url]
[![Unit Tests][travis-img]][travis-url]
[![Coverage Tests][codecov-img]][codecov-url]

Python VR is a **free** *virtual reality* game for programming python, designed to teach beginners the fundamentals of programming, let intermediate/advanced developers to compete in daily programming competitions, explore existing codebases as if they were a real place, and collaborate with other programmers whether they're on their favorite Text Editor/IDE or in the game.

## Architecture

Python VR is built on top of **Rust**, **Vulkan**, **OpenVR**, and **Language Servers**.

The application is divided into the main application that handles rendering/networking/gameplay/mods, a language server that conforms to the [Language Server Protocol Spec](https://github.com/Microsoft/language-server-protocol) that could theoretically be swapped with other languages in the future, and a Language VR Server to send language specific VR metadata along side the language server.

| | Screenshots  | |
|:--:|:--:|:--:|
| ![][screenshot-1] | ![][screenshot-2] | ![][screenshot-3] |

## Sponsors

![Sponsors](docs/images/brand/sponsors.png)

This project was sponsored by the [OpenHID Lab](http://openhid.com), an *HCI research lab* part of the [High Performance Database Research Center](http://hpdrc.fiu.edu/) at Florida International University.

[screenshot-1]: docs/images/screenshots/0.png
[screenshot-2]: docs/images/screenshots/1.png
[screenshot-3]: docs/images/screenshots/2.png

[license-img]: http://img.shields.io/:license-mit-blue.svg?style=flat-square
[license-url]: https://opensource.org/licenses/MIT
[travis-img]: https://img.shields.io/travis/alaingalvan/python-vr.svg?style=flat-square
[travis-url]:https://travis-ci.org/alaingalvan/python-vr
[codecov-img]:https://img.shields.io/codecov/c/github/alaingalvan/python-vr.svg?style=flat-square
[codecov-url]: https://codecov.io/gh/alaingalvan/python-vr