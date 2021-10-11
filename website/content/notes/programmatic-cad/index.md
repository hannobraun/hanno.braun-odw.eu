+++
title = "Programmatic CAD"
date = 2021-09-16
+++

{{ ext_link(url="https://en.wikipedia.org/wiki/Computer-aided_design", text="CAD") }} software usually comes with a graphical user interface, but that is not the only way to approach that problem. It's also possible to write a CAD model as a computer program, either using a specialized programming language, or a general-purpose language with an appropriate API.

<!-- more -->

As a software developer, programmatic CAD makes complete sense to me. You can use established tools for programming and get all the advantages of a programming language built-in. Collaboration with other team members, adding comments to document aspects of your model, automating repetitive modelling tasks, generative design... all this and more is available without needing to be explicitly supported by the CAD software.

The remainder of this note presents some interesting examples of programmatic CAD software that I'm aware of, both for my own future reference, and in the hope that it might be useful for others too. If you think that I've missed or misrepresented something, [please let me know](/contact)!


### OpenSCAD

{{ ext_link(url="https://openscad.org/", text="OpenSCAD") }} is well-known in the open source CAD space. It's been around for a while and is widely used.

Advantages:
- Mature. Very solid solution for many kinds of models.
- Large community, easy to get into.
- Can export to a large variety of file formats.

Disadvantages:
- Uses a custom language that can be very weird, for example around variable scoping, or the implicit nature of geometry.
- Limited feature set. Let's you generate geometry based on constructive solid geometry, but offers little else.
- Built-in library that is very basic. 3rd-party libraries are available, but need to be managed manually by copying files around. It can also be hard to figure out what is suitable to your needs.

I've done most of my CAD modelling with OpenSCAD. Overall, I'm not too fond of it, but I still keep coming back to it, and it's the one I compare all the others to, which says a lot.


### libfive

{{ ext_link(url="https://libfive.com/", text="libfive") }} seems less well-known than OpenSCAD, but looks promising. It uses {{ ext_link(url="https://en.wikipedia.org/wiki/Function_representation", text="function representation") }}, rather than the polyhedron-based approach of OpenSCAD.

Advantages:
- Looks really good on paper.
- Has a GUI interface, but also available as a library (C-compatible interface, so usable from lots of languages).
- Supports limited editing through the GUI.
- Comes with support for Guile and Python as modelling languages.
- Seems to do a great job rendering sharp corners, which is not a given when using function representation.

Disadvantages:
- Haven't used it seriously yet, so unclear how good it really is.
- Only supports exporting to STL.
- Not as widely packaged as OpenSCAD, and compiling it yourself can be problematic.

I'm currently looking to get into it more, maybe using it as my main CAD program going forward.


### CadQuery

{{ ext_link(url="https://cadquery.readthedocs.io/", text="CadQuery") }} is written in Python and based on an established CAD kernel.

Advantages:
- Looks really good on paper.
- Uses Python as the modelling language.
- Based on Open CASCADE, which is more advanced than OpenSCAD's kernel.

Disadvantages:
- Haven't used it yet.
- Not as widely packages as OpenSCAD.


### ImplicitCAD

{{ ext_link(url="https://implicitcad.org/", text="ImplicitCAD") }} is an alternative to OpenSCAD that uses function representation under the hood, like libfive.

Advantages:
- Backwards-compatible with OpenSCAD (mostly?).
- Also available as a Haskell library.
- Is a native application, but browser-based editor available on its website.

Disadvantages:
- Haven't used it, except for the browser-based editor here or there.
- Doesn't do a great job rendering geometry, specifically around sharp edges.
- Not as widely packaged as OpenSCAD, and compiling it yourself can be problematic.


### Cascade Studio

{{ ext_link(url="https://zalo.github.io/CascadeStudio/", text="Cascade Studio") }} is browser-based and uses JavaScript as the modelling language.

Advantages:
- Browser-based, which is a convenient capability to have.
- Uses JavaScript as the modelling language.
- Based on Open CASCADE.

Disadvantages:
- Haven't used it seriously.
- Not available as a native application.
- Uses JavaScript as the modelling language.


### sdfx

{{ ext_link(url="https://github.com/deadsy/sdfx", text="sdfx") }} is another not-that-well-known contender, written in Go.

Advantages:
- Screenshots look really promising.
- Uses Go as the modelling language. Not a huge fan, but with static typing and fast compile times, it sounds like a great fit for this use case.

Disadvantages:
- Haven't tried it.


### Fornjot

Last, and also least, my own effort, {{ ext_link(url="https://github.com/hannobraun/fornjot", text="Fornjot") }}. It uses function representation and is written in Rust.

Advantages:
- Written by me, in Rust, which is lots of fun.
- I literally know it inside out.
- Uses Rust as the modelling language, which is fun, powerful, and reliable.

Disadvantages:
- Experimental, incomplete, basically useless.
- Uses Rust as the modelling language, which compiles way too slowly for rapid iteration.
