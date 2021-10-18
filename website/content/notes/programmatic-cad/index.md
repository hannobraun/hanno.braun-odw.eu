+++
title = "Programmatic CAD"
date = 2021-09-16
updated = 2021-10-13

[extra]
intro = """
CAD software usually comes with a graphical user interface, but that is not the only way to approach that problem. It's also possible to write a CAD model as a computer program, either using a specialized programming language, or a general-purpose language with an appropriate API. This approach is called programmatic CAD or Code-CAD.
"""
+++

As a software developer, programmatic CAD makes complete sense to me. You can use established tools for programming and get all the advantages of a programming language built-in. Collaboration with other team members, adding comments to document aspects of your model, automating repetitive modelling tasks, generative design... all this and more is available without needing to be explicitly supported by the CAD software.

This note represents a snapshot of my research into this topic. I'm publishing it both for my own future reference, and in the hope that it might be useful for others too. If you think that I've missed or misrepresented something, [please let me know](/contact)!

### CAD Kernels

A CAD kernel is a software library that provides the core data structures and algorithms required for building CAD software. While some CAD programs come with their own custom kernel (or might not make such a distinction), there are some independently developed kernels that can be used to build CAD programs on top off.

While building your own CAD program is a significant task, especially if it is meant to be generally usable, it might make sense to build a specialized one. For example to enable designs using some specialized system of components, or to support a customizable product.

I've restricted my research to open source CAD kernels.

#### Open CASCADE Technology (OCCT)

{{ ext_link(url="https://www.opencascade.com/open-cascade-technology/", text="OCCT") }} is a widely used kernel, being the basis for the well-known FreeCAD, as well as some of the CAD programs presented in the next section.

#### CGAL

{{ ext_link(url="https://www.cgal.org/", text="CGAL") }} is another example of a widely used CAD kernel, being the basis for OpenSCAD (see next section). It seems to be less advanced than OCCT, providing fewer features.

#### libfive

{{ ext_link(url="https://libfive.com/", text="libfive") }} seems to be less widely-used than OCCT or CGAL, but is still very interesting. In contrast to those other two, it is based on {{ ext_link(url="https://en.wikipedia.org/wiki/Function_representation", text="function representation") }}, rather than a more traditional approach.


### CAD Programs

CAD programs (sometimes called CAD packages), are intended for use by end users, as opposed to the CAD kernels they build on. Some of the CAD programs presented here are based on the aforementioned CAD kernels, others use their own custom code to handle geometry.

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


### libfive Studio

{{ ext_link(url="https://libfive.com/studio/", text="libfive Studio") }} seems less well-known than OpenSCAD, but looks promising. It is built on the libfive kernel, and made by the same author.

Advantages:
- Looks really good on paper.
- Supports limited editing through the GUI.
- Comes with support for Guile and Python as modelling languages.
- Seems to do a great job rendering sharp corners, which is not a given when using function representation.

Disadvantages:
- Haven't used it seriously yet, so unclear how good it really is.
- Only supports exporting to STL.
- Not as widely packaged as OpenSCAD, and compiling it yourself can be problematic.

I'm currently looking to get into it more, maybe using it as my main CAD program going forward.


### CadQuery

{{ ext_link(url="https://cadquery.readthedocs.io/", text="CadQuery") }} is written in Python and based on OCCT.

Advantages:
- Looks really good on paper.
- Uses Python as the modelling language.
- Based on OCCT, which is more advanced than OpenSCAD's CGAL.

Disadvantages:
- Haven't used it yet.
- Not as widely packaged as OpenSCAD.


### ImplicitCAD

{{ ext_link(url="https://implicitcad.org/", text="ImplicitCAD") }} is an alternative to OpenSCAD that uses function representation under the hood, like libfive.

Advantages:
- Backwards-compatible with OpenSCAD (mostly?).
- Also available as a Haskell library.
- Is a native application, but browser-based editor available on its website.

Disadvantages:
- Haven't used it, except for the browser-based editor here or there.
- Doesn't do a great job rendering geometry, specifically around sharp corners.
- Not as widely packaged as OpenSCAD, and compiling it yourself can be problematic.


### Cascade Studio

{{ ext_link(url="https://github.com/zalo/CascadeStudio", text="Cascade Studio") }} is browser-based and uses JavaScript as the modelling language.

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

Last, and also least, my own effort, {{ ext_link(url="https://github.com/hannobraun/fornjot", text="Fornjot") }}. It experiments with both function representation and more traditional approaches based on triangle meshes, and is written in Rust.

Advantages:
- Written by me, in Rust, which is lots of fun.
- I literally know it inside out.
- Uses Rust as the modelling language, which is fun, powerful, and reliable.

Disadvantages:
- Experimental, incomplete, basically useless.
- Uses Rust as the modelling language, which compiles way too slowly for rapid iteration.
