+++
title = "3D Printer Issues: Clogged Nozzle"
date  = 2021-11-08

[extra]
image = "discoloration.jpg"
intro = """
After printing perfectly fine for over a year, my Prusa Mini experienced issues, to the point where it became basically unusable for a few weeks. In this note, I'm going to write about the symptoms I saw, what I think caused them, how I overcame the problems, and how I intend to prevent them from happening again in the future.
"""
+++

I believe that my problems were caused by partial clogging, on two different occasions, for two different reason. While I'm pretty sure that this is what happened, it is totally possible that I misinterpreted the situation and the actual problems were totally different.

Please exercise the appropriate amount of caution, and if you have some insight, please [let me know](/contact). I'm happy to update this note later, as new information becomes available.


### Why I'm Posting This

Even though the problems (and solutions) ended up being pretty basic, I had a hard time diagnosing them. It took weeks of experimentation, research, and in the end a bit of luck, to get everything sorted.

The pictures I found online, regarding what the result of partial clogging might look like, didn't match what I was seeing, so I followed up on other possible solutions instead.

I was probably just looking in the wrong places, but I figure, it won't hurt to add more data points. Maybe it will help someone who's googling frantically, trying to get their printer problems sorted.


### First Clog: Discoloration, Weird Lumps

#### The Problems

After switching filaments from a dark-colored PLA to [Fillamentum NonOilen](/notes/fillamentum-nonoilen-on-prusa-mini/), I saw discoloring on the first few prints.

{{
    preview_image(
        path="/notes/3d-printer-issues-clogged-nozzle/discoloration.jpg",
        alt="A 3D printed calibration cube, mainly white, but showing a dark discoloration on the bottom."
    )
}}

It was pretty obvious then that there still was PLA left in the hot end. But the problem disappeared over the next few prints, so I thought it had all cleared out and went on with my life.

Then I started noticing that the bottoms of my prints didn't look like I expected it to. They were showing an unexpected taper (more so than the slicer settings should have caused) and weird lumps.

{{
    preview_image(
        path="/notes/3d-printer-issues-clogged-nozzle/lump.jpg",
        alt="A 3D-printed calibration cube, posed in front of a light source for better visibility of its problematic features. Its first few layers show a taper, starting small, before increasing to the expected size. One size shows a bulge, where layers are too large."
    )
}}

#### How I Fixed It

After becoming convinced that this was caused by a partial clog, I did a series of cold pulls, following the {{ ext_link(text="instructions from Prusa", url="https://help.prusa3d.com/en/article/cold-pull-mini_126399") }}. Prints were fine after that.

#### How To Prevent It

NonOilen prints at a lower temperature than PLA (180-185°C to 215°C in my case). This lower printing temperature made the clogging basically unavoidable. The PLA that was still in there would stay viscous, not fully getting pushed out by the NonOilen that followed it.

I should have made sure that the nozzle was clean before loading the NonOilen, by doing a few cold pulls, using cleaning filament, or at least by heating the nozzle up and pushing the PLA out with the NonOilen.

This should hold true for any switch from a higher-temperature to a lower-temperature filament.


### Second Clog: Splurging

#### The Problem

After having run the Mini's built-in first-layer calibration procedure with a 0.25mm nozzle, I saw these weird splurges pretty regularly. As if the nozzle was building up too mush pressure, which then had to get released quickly.

{{
    preview_image(
        path="/notes/3d-printer-issues-clogged-nozzle/splurge.jpg",
        alt="A rectangular one-layer calibration print on a smooth print sheet. The skirt shows a thick blob where much more material has been extruded than expected."
    )
}}

#### How I Fixed It

While cleaning the outside of the nozzle, having heated it up to do so, I followed a whim and heated the nozzle to maximum temperature, figuring that if something was in there blocking the nozzle, it might just flow out at high temperature.

Quite a bit of material came out, then some fumes. Once nothing was happening anymore, I stopped. The splurges no longer happened afterwards.

#### How To Prevent It

The Prusa Mini's built-in first-layer calibration procedure assumes that the default 0.4mm nozzle is equipped. Running it with the 0.25mm nozzle caused it to push more material through than intended.

I should not have used the built-in calibration procedure in this case. In fact, I'm not using it at all anymore, and have come up with something better. I plan to write about that soon.


### Conclusion

3D Printing can be pretty easy, especially if you have a solid new printer that you are using only for PLA. I found out that you can run into trouble pretty quickly once you start moving away from that.

I came out of this with a new appreciation for the printing process. What goes into it, and what I can do to influence it. I hope I'll continue to improve my understanding, so I can diagnose and fix these kinds of problems more quickly, and less haphazardly, in the future.
