+++
title = "Fillamentum NonOilen on Prusa Mini"
date  = 2021-11-11

[extra]
image = "test-prints.jpg"
intro = """
Fillamentum NonOilen is an exciting new material that promises better biodegradability than PLA, high temperature resistance, and other advantageous attributes. I figured out how to print it on the Prusa Mini.
"""
+++

I'm interested in manufacturing and selling my own 3D-printed products, but I don't want to add to the numerous problems being caused by plastic in our world. A [biodegradable filament](/notes/biodegradable-filaments/) seems like the right solution, and {{ ext_link(text="Fillamentum NonOilen", url="https://www.fillamentumnonoilen.com/") }} looks the most promising.

I want to print NonOilen on my Prusa Mini, but there is no built-in profile in PrusaSlicer (not a surprise, for such a young product). I've never created a slicer profile for a Filament before, but I figured I'd give it a try, adapting the built-in PLA profile by following the {{ ext_link(text="NonOilen 3D printing guide", url="https://fillamentum.com/wp-content/uploads/2021/01/FILL_Printing_Guide_NonOilen.pdf") }}.

<a
    class="download"
    href="/notes/fillamentum-nonoilen-on-prusa-mini/Fillamentum%20NonOilen.ini">
    Download PrusaSlicer profile for NonOilen
</a>

### Summary

In case you're in a hurry, here's the short summary:

- Use adhesive! I've had success with 3DLAC.
- I recommend the textured PEI sheet.
- Use nozzle temperature of 185°C and bed temperature of 50°C.
- Set the extrusion multiplier to 1.05.
- Use normal PLA settings for everything else.

### Adhesive

NonOilen poses a weird challenge: On the one hand, it can stick extremely well to the smooth PEI sheet, especially if you print only one layer. On the other hand, any real print will either be dragged around by the nozzle, sooner or later (if it's small) or show significant warping (if it's larger).

{{
    preview_image(
        path="/notes/fillamentum-nonoilen-on-prusa-mini/warping.jpg",
        alt="An aborted NonOilen print, showing significant warping."
    )
}}


I've seen {{ ext_link(text="speculation on Reddit", url="https://www.reddit.com/r/3Dprinting/comments/mpntmy/anyone_tried_nonoilen_filament/gv2ol8j/") }}, that this is really just a warping problem that shows up in different ways, and I concur. At least this is the only way these observations make sense for me.

Fillamentum recommends printing with an adhesive, and that addresses both problems: Causing the print to stick, which also providing separation between print and bed, allowing it to be removed without damaging the bed.

I've tried to get around it, but without any luck. Maybe it's possible with an enclosure, but in the meantime, I've had great success with {{ ext_link(text="3DLAC", url="https://www.3dlac.com/") }}.


### Sheet

As mentioned above, NonOilen can stick really well to the smooth PEI sheet. This can pose a problem, if you don't apply adhesive everywhere you end up printing, or forget completely.

I've had great success with the textured PEI sheet. It's easy to remove NonOilen prints from that after you let it cool down, but they still stick great with 3DLAC.


### Temperatures

There's not much to say here. The printing guide gives a range of 175°C - 195°C for the nozzle temperature. I chose 185°C, directly in the middle.

Bed temperature range is given as 0°C - 50°C. I chose the upper end at 50°C.


### Extrusion Multiplier

Testing with a vase mode print turned out that the single-perimeter wall thickness was slightly too small. I've adjusted the extrusion multiplier to a value of 1.05, which made it more accurate.


### Results

Following the guidelines I've laid out here, printing Fillamentum NonOilen on the Prusa Mini has been pleasant and reliable. However, I've yet to print anything serious with it, beyond the various test prints required to dial in these settings.

I intend to use it for some upcoming projects. If that requires any further adjustments, I will update this note with my latest findings.

{{
    preview_image(
        path="/notes/fillamentum-nonoilen-on-prusa-mini/test-prints.jpg",
        alt="Three successful test prints, printed with Fillamentum NonOilen. One larger cube, printed in vase mode, one calibration cube, and a one-layer square."
    )
}}
