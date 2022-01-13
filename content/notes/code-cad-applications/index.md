+++
title   = "Code-CAD Applications"
date    = 2021-09-16
updated = 2022-01-17

[extra]
intro = """
CAD software usually comes with a graphical user interface, but this is not the only approach. It's possible to write a CAD model as a computer program, either using a specialized programming language, or a general-purpose language with an appropriate API. This approach is called Code-CAD or programmatic CAD.
"""

[[extra.changes]]
date = 2021-10-13
text = """
Added separate section about CAD kernels; updated section about programs to refer back to it. Other minor tweaks.
"""

[[extra.changes]]
date = 2022-01-17
text = """
Renamed to "Code-CAD Applications" (from "Programmatic CAD"). Updated a bunch of things all over. Added JSCAD.
"""
+++

As a software developer, Code-CAD makes complete sense to me. You get all the power of a programming language and you can use all the established tools for programming. Creating intricate models with no repetitive work, collaborating with other team members, adding comments to document your model... [all this and more](/notes/code-cad-advantages) is available without needing to be explicitly supported by the CAD software.

I'm always on the look-out for applications using this approach, and this note represents a snapshot of my research into this topic. I'm publishing it both for my own future reference, and in the hope that it might be useful for others too. If you think that I've missed or misrepresented something, [please let me know](/contact)!

### CAD Kernels

A CAD kernel is a software library that provides the core data structures and algorithms required for building CAD software. While some CAD programs come with their own custom kernel (or might not make such a distinction), there are some independently developed kernels that CAD programs can build on on top of.

I've restricted my research to open source CAD kernels.

#### Open CASCADE Technology (OCCT)

{{ ext_link(url="https://www.opencascade.com/open-cascade-technology/", text="OCCT") }} is a widely used kernel, being the basis for the well-known FreeCAD, as well as some of the CAD programs presented in the next section.

#### CGAL

{{ ext_link(url="https://www.cgal.org/", text="CGAL") }} is another example of a widely used CAD kernel, being the basis for OpenSCAD (see next section). It seems to be less advanced than OCCT, providing fewer features.

#### libfive

{{ ext_link(url="https://libfive.com/", text="libfive") }} seems to be less widely-used than OCCT or CGAL, but is still very interesting. In contrast to those other two, it is based on {{ ext_link(url="https://en.wikipedia.org/wiki/Function_representation", text="function representation") }}.


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
- Limited feature set. Lets you generate geometry based on constructive solid geometry, but offers little else.
- Built-in library that is very basic. 3rd-party libraries are available, but need to be managed manually, by copying files around. It can also be hard to figure out which ones are suitable to your needs.

I've done most of my CAD modeling with OpenSCAD. Overall, I'm not too fond of it, but I still keep coming back to it, and it's the one I compare all the others to, which says a lot.

### libfive Studio

{{ ext_link(url="https://libfive.com/studio/", text="libfive Studio") }} seems less well-known than OpenSCAD. It is built on the libfive kernel, and made by the same author.

Advantages:
- Supports limited editing through the GUI.
- Comes with support for Guile and Python as modeling languages.
- Seems to do a great job rendering sharp edges, which is not a given when using function representation.

Disadvantages:
- Limited feature set. Not much there beyond Constructive Solid Geometry.
- Editor is lacking lots of small convenience features compared to more powerful editors.
- Only supports exporting to STL.
- Not as widely packaged as OpenSCAD, and compiling it yourself can be problematic.

I've used it {{ ext_link(text="for one project", url="https://github.com/hannobraun/prusa-mini-enclosure") }} and it worked well enough. In the end, I found no good reason to keep using it over OpenSCAD.

### CadQuery

{{ ext_link(url="https://cadquery.readthedocs.io/", text="CadQuery") }} is written in Python and based on OCCT.

Advantages:
- Uses Python as the modeling language.
- Based on OCCT, which is more advanced than OpenSCAD's CGAL.

Disadvantages:
- Haven't used it yet.
- Not as widely packaged as OpenSCAD.

I tried to use it on multiple occasions, but it's not packaged for Arch Linux (which is what I use), and compiling it from source always failed for one reason or another.

### ImplicitCAD

{{ ext_link(url="https://implicitcad.org/", text="ImplicitCAD") }} is an alternative to OpenSCAD that uses function representation under the hood, like libfive.

Advantages:
- Backwards-compatible with OpenSCAD (mostly?).
- Also available as a Haskell library.
- Is a native application, but a browser-based editor is available on its website.

Disadvantages:
- Haven't really used it, except for the browser-based editor here or there.
- Doesn't do a great job rendering geometry, specifically around sharp corners.
- Not as widely packaged as OpenSCAD, and compiling it yourself can be problematic.


### Cascade Studio

{{ ext_link(url="https://github.com/zalo/CascadeStudio", text="Cascade Studio") }} is browser-based and uses JavaScript as the modeling language.

Advantages:
- Browser-based, which is a convenient capability to have.
- Uses JavaScript as the modeling language.
- Based on OCCT.

Disadvantages:
- Haven't used it seriously.
- Not available as a native application.
- Uses JavaScript as the modeling language.


### sdfx

{{ ext_link(url="https://github.com/deadsy/sdfx", text="sdfx") }} is another not-that-well-known contender, written in Go. It also uses function representation.

Advantages:
- Uses Go as the modeling language. Not a huge fan, but with static typing and fast compile times, it sounds like a great fit for this use case.
- Has recently received updates that improve the rendering of sharp edges.

Disadvantages:
- Haven't tried it.


### JSCAD

{{ ext_link(url="https://github.com/jscad/OpenJSCAD.org", text="JSCAD") }} is a CAD application written in JavaScript.

Advantages:
- Has been consistently developed for many years.
- Documentation looks good.
- Runs in browsers.
- Uses JavaScript as the modeling language.

Disadvantages:
- Haven't tried it.
- Limited feature set. Seems to be roughly on par with OpenSCAD.
- Uses JavaScript as the modeling language.


### Fornjot

Last, and also least, my own effort, {{ ext_link(url="https://github.com/hannobraun/fornjot", text="Fornjot") }}. After experimenting with function representation and other techniques for a while, I finally settled on a more traditional approach based on boundary representation.

Advantages:
- Written by me, in Rust, which is lots of fun.
- I literally know it inside out.
- Uses Rust as the modeling language, which is fun, powerful, and reliable.
- Found a way to keep compile times down, to make using Rust as a modeling language practical (at least so far).

Disadvantages:
- Experimental, incomplete, at this point still basically useless.
