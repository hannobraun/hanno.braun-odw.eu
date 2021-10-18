+++
title = "Filing Table"
date = 2020-08-25

[extra]
image = "notes/filing-table/filing-table.jpg"
+++

When I got my 3D printer, it radically improved my workshop's capabilities. Suddenly I could build things at a previously unimaginable accuracy. It also introduced a disconnect though: While I could form plastic to tenths of millimeters, my aluminium pieces were still crooked.

Since I (currently) have neither the room nor the budget for a CNC mill, this got me thinking: Can I somehow use the accuracy of the printer to improve my accuracy when working with aluminium? I'm sure the last word on this topic isn't spoken yet, but here's my first attempt at an answer.

{{
    preview_image(
        path="/notes/filing-table/filing-table.jpg",
        alt="The filing table, on it a piece of aluminium and a file in a file holder."
    )
}}

The principle is easy: A file is put in a specially made file holder, which is dimensioned to fit into the notch on the filing table. The file in its holder can then be moved along a defined axis. On the filing table is a guide that is orthogonal to that axis.

{{
    video(
        path="/notes/filing-table/filing-table.webm",
        text="Me using the filing table to work on a piece of aluminium, showing some progress on the surface."
    )
}}

The results are pretty good. I can make pieces of aluminium now that have flat faces and right angles, whereas before I could do that only very approximately.

{{
    video(
        path="/notes/filing-table/end-result.webm",
        text="The end result of some filing using the filing table. Surfaces are mostly flat, and angles are mostly right, based on some eyeball-based inspection."
    )
}}

Granted, the result in that video looks a bit better than actually is in reality. If I had held the piece and the square against a light source, it would have shown the imperfections. Still, this is way better than what I was able to do before.

So what now? Obviously, this isn't built to last. The table and the file holder already show wear after using them a few times. I don't have any suitable lubrication at hand, and the fine aluminium dust getting between the moving surfaces certainly doesn't help. It is a great proof of concept though, and I'll be looking into using this (or an improved version) to bootstrap an aluminium version.

All the modeling was done with OpenSCAD. I've made the <a href="filing-table.zip">models available for download</a>, if you want to print one yourself.
