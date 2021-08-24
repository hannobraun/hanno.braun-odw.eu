+++
title = "3D-Printed Screws"
date = 2020-08-18

[extra]
image = "notes/3d-printed-screws/big-screw.jpg"
+++

When I first learned that 3D printers can print screws, my reaction was one of mild surprise: Yes, why wouldn't you be able to print very rough screw threads with a 3D printer?

Some time later, my Prusa Mini arrived, and I discovered that the thumb drive it came with had models of a screw and a nut on it. I wanted to try that out.

{{
    preview_image(
        path="notes/3d-printed-screws/big-screw.jpg",
        alt="A big 3D-printed screw, placed on a hand for size comparison."
    )
}}

The result looked so perfect that I started wondering: This big screw could certainly not be the limit of what this printer is capable of. I wondered how much smaller I could go, while still getting functional screw threads.

So I started scaling down the {{ external_link(url="https://www.prusaprinters.org/prints/13786-screw-and-nut", text="original model") }}. Here's the end result:

{{
    preview_image(
        path="notes/3d-printed-screws/screw-collection.jpg",
        alt="The big screw and nut from before in the background, with three smaller screw/nut combinations in front of it."
    )
}}

I scaled the model down in PrusaSlicer, halving all dimensions each time. This resulted in 3 new screw/nut combinations: Half, quarter and one eighth the size of the original, respectively.

Unfortunately I didn't take notes or save the 3MF files, so the following is from memory:

- I printed the original with PrusaSlicer's default "0.15mm QUALITY" profile. The half-size screw/nut worked great with the same settings.
- To get the quarter-size screw/nut to work, I had to adjust the layer height to 0.10 mm. This actually resulted in the nicest-feeling result. The fit between the original models is quite sloppy, and since I just scaled it down (without adjusting the thread tolerances), this just happened to result in the nicest fit at quarter size.
- For the eight-size screw/nut, I adjusted the layer height to 0.05 mm. This one feels and works quite well too.

I realize that none of this is going to be news for experienced printers, but I wasn't even remotely aware that it is possible to get this kind of detail out of a printer that costs 380€. I aways knew that a 3D printer was going to be useful in the workshop, but this opens up many more possibilities.
