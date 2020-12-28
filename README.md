# Propagation

A supersonic cross platform synchronous ASIO device manager and audio routing
application.

## Cool description and all, but what is Propagation?

Propagation is a multiplatform audio driver that enables audiophiles to route
application audio through any DAW (Digital Audio Workstation) of their choice.

- All audio streams created by Propagation are synchronized to a physical audio
interface to mitigate clock drift and buffer underrun problems.

- Distinct applications can be routed to Propagation created endpoints. This
allows for the virtual passthrough of audio that can be captured by recording
devices and software alike.

- The routing of application audio can be controlled given specific system
directory paths. All applications under a certain directory and its subdirectory
can be routed to a specific endpoint. For instance, when the steamapps directory
is specified then every application under that directory can be routed to a
singular endpoint.

## Installation

TBD

### Prerequisites 

TBD

## Contributing

Don't hesitate to contribute to the repository, even if you're brand new to
programming or new to the Rust ecosystem. We would still like your help. If
you're not sure where to start try creating a [good first issue](https://github.com/dezzyne/propagation/labels/good%20first%20issue).

If you would like to contribute to Propagation, please take a peek at our
[Contributing Guide](https://github.com/dezzyne/propagation/blob/master/CONTRIBUTING.md).
Also, if you have any specific questions about Propagation visit the [Dezzyne Discord Server](https://discord.gg/ZmSpJSq).

(NEED TO FIX THE DISCORD LINK ABOVE)

## Inspired By

All of these third party applications have been the main inspiration for Propagation.
When you have the chance, give them a look over.

- **[Synchronous Audio Router](https://github.com/eiz/SynchronousAudioRouter)** - A low latency application audio router for Windows.
- **[VB-Audio VoiceMeeter Banana](https://www.vb-audio.com/Voicemeeter/banana.htm)** - A virtual audio mixer for Windows.
- **[CheVolume](http://www.chevolume.com/)** - A virtual audio router for Windows.

## License

Copyright © 2020 - Present, [The Dezzyne Team and the repo's Contributors](https://github.com/dezzyne/propagation/graphs/contributors).

This project is dual licensed with the [MIT](https://github.com/dezzyne/propagation/LICENSE-MIT) and [APACHE 2.0](https://github.com/dezzyne/propagation/LICENSE-APACHE) licenses.
