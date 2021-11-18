+++
title = "Spacer"
date  = 2021-11-18

[extra]
image = "spacer.png"
intro = """
This is a proof of concept for providing access to a 3MF file, exported from a customizable CAD model, through a web page.
"""
+++

{{
    preview_image(
        path="/notes/spacer/spacer.png",
        alt="CAD model of a simple spacer"
    )
}}

A spacer is used to create a distance between two connected parts in an assembly. It has a hole in the middle, so a bolt can go through. If you want to print your own, you can use the following form to configure the spacer and download a 3MF file for printing.

<form method="GET" action="/api/models/spacer.3mf">
    <input
        type="hidden"
        name="rev"
        value="1" />
    <div>
        <label for="outer">Outer diameter (in mm)</label>
        <input
            type="number"
            required
            name="outer"
            id="outer"
            step="0.01"
            value="8.0"
            placeholder="outer diameter (in mm)" />
    </div>
    <div>
        <label for="inner">Inner diameter (in mm)</label>
        <input
            type="number"
            required
            name="inner"
            id="inner"
            step="0.01"
            value="5.0"
            placeholder="inner diameter (in mm)" />
    </div>
    <div>
        <label for="height">Height (in mm)</label>
        <input
            type="number"
            required
            name="height"
            id="height"
            step="0.01"
            value="5.0"
            placeholder="height (in mm)" />
    </div>
    <input
        type="submit"
        value="Download model as 3MF" />
</form>

The spacer has been modeled in OpenSCAD, and this proof of concept can be extended for any other OpenSCAD model; although the time it takes to export the 3MF file might rise considerably, for a more complex model.

The full code for the model and the API that is used to integrate it into this website is open source. {{ ext_link(text="Check out the repository on GitHub", url="https://github.com/hannobraun/model-api") }}, and feel free to use it for your own models.

If you have any questions or comments, [please let me know](/contact)!
