+++
title = "Dovetail Slide Box"
date = 2020-12-26

[extra]
image = "box-open.jpg"
+++

Sometimes you need a box to put stuff in.

{{
    preview_image(
        path="/notes/dovetail-slide-box/box-open-with-contents.jpg",
        alt="An open box with a dial indicator inside it."
    )
}}

In this case, I needed something to keep my dial indicator in, so I printed a custom box for it. Here's the box again without the dial indicator in it.

{{
    preview_image(
        path="/notes/dovetail-slide-box/box-open.jpg"
        alt="An open, empty box. It's made to fit a dial indicator."
    )
}}

The lid slides onto the box with a dovetail slide mechanism. This works pretty well, but can also be a bit fiddly.

{{
    preview_image(
        path="/notes/dovetail-slide-box/box-closed.jpg"
        alt="A closed box."
    )
}}

I've uploaded [my OpenSCAD model](dovetail-slide-box.zip). The archive contains a file that defines a generic box of customizable size, and another file that uses that to define the dial indicator box.

There are some problems with it. The top of the box came out about 0.5 mm longer than the bottom (and the lid), which prevents the detent that's supposed to lock the lid in place from doing anything. At this size, there's enough friction to hold everything in place, so it ends up working anyway.

There's also a clearly visible line at the outside of the box, which I believe is an instance of [the Benchy hull line](https://www.help.prusa3d.com/cs/article/the-benchy-hull-line_124746). Maybe those two problems are related, but I don't know.
