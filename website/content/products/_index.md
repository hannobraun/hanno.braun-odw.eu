+++
title    = "Products"
template = "products.html"
+++

## Spacer

{{
    preview_image(
        path="products/spacer.png",
        alt="CAD model of a simple spacer"
    )
}}

A spacer is used two create a distance between two connected parts in an assembly. It has a hole in the middle, so a bolt can go through. Often enough, an off-the-shelf spacer or a stack of washers will do the job well enough. Sometimes though, you need a spacer with very precise dimensions. Then a custom, 3D-printed one comes in handy.


### Print your own

Configure a custom spacer and download it for 3D printing using the following form.

<form method="GET" action="https://model-api.braun-odw.eu/models/spacer.3mf">
    <div>
        <label for="outer">Outer radius (in mm)</label>
        <input
            type="number"
            required
            name="outer"
            id="outer"
            step="0.01"
            value="30.0"
            placeholder="outer radius (in mm)" />
    </div>
    <div>
        <label for="inner">Inner radius (in mm)</label>
        <input
            type="number"
            required
            name="inner"
            id="inner"
            step="0.01"
            value="15.0"
            placeholder="inner radius (in mm)" />
    </div>
    <div>
        <label for="height">Height (in mm)</label>
        <input
            type="number"
            required
            name="height"
            id="height"
            step="0.01"
            value="10.0"
            placeholder="height (in mm)" />
    </div>
    <input
        type="submit"
        value="Download model as 3MF" />
</form>


### Buy printed part

This part will soon be available for sale (although you should really print your own, if you have a printer available). Check back later, if you're interested, or sign up to the newsletter below to be notified!
