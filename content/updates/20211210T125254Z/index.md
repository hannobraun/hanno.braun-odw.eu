+++
title = "20211210T125254Z"
date  = "2021-12-10T12:52:54Z"
+++

{{
    preview_image(
        path="/updates/20211210T125254Z/screw-pad.jpg",
        alt="The Screw Pad, a height-adjustable support for light workshop duties."
    )
}}

I've printed the first prototype of the product I'm working on: A height-adjustable support for light workshop duties. I'm calling it the Screw Pad (working title).

The idea is to create something simple, that I can design and manufacture easily, so I can learn about all the other aspects of selling a physical product. Learning is the priority here. I don't expect to sell that many (or any 😅).

Despite choosing something simple, the design was a huge pain. Because of the screw thread. FreeCAD can do helical extrudes, of course, but it's buggy and I couldn't get it to work. SolveSpace can also do it, but support seems half-baked and weird. Commercial options don't run on Linux or are out of my price range.

I finally managed to do it with {{ ext_link(text="BOSL", url="https://github.com/revarbat/BOSL") }}. Despite my best efforts, I always keep coming back to OpenSCAD 😄
