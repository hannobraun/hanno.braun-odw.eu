+++
title   = "My Criteria for CAD Software"
date    = 2021-10-26
updated = 2022-01-10

[extra]
intro = """
I've been doing CAD modeling for a while now, and although I'm far from being an expert, I have developed opinions on how CAD software should work, and what it should provide. In this note, I lay out the criteria by which I evaluate CAD software.
"""

[[extra.changes]]
date = 2021-10-28
text = """
Added project health as criterion.
"""

[[extra.changes]]
date = 2021-11-01
text = """
Replaced criterion about 2D sketches with more general criterion about user experience.
"""

[[extra.changes]]
date = 2022-01-10
text = """
Major update. Added new criteria, merged and updated some existing ones, and changed their order.
"""
+++

Before I start, I'd like to emphasize two things:

- First, as the title states, these are *my* criteria. I'm not telling you what to think or do. I'm just describing what I think would best fit my own use cases and preferences.
- Second, these criteria are neither complete nor final. As I gain more experience and learn more about CAD, I'm sure to think of more to add here, or change my mind about some of them.

But enough prelude. Let's get into it.


### Criteria

#### Overview

Before I go into detail, here's a short overview of the criteria I have come up with so far:

1. The software should be developed by a **healthy project**. Ideally, it is *open source*, *actively developed*, has a *vibrant community*, *documentation* is available, and it is *packaged* for a wide variety of platforms.
1. Models should be **defined as code**. The language that code is written in should be *easy to use*, *powerful*, and *robust*.
1. **Modeling features**, like *Constructive Solid Geometry*, *extrude/sweep operations*, ability to *reference and manipulate existing geometry*, *constraint-based modeling*, and *fillets/chamfers* should be available.
1. The software should be conducive to working **conveniently and efficiently**.
1. The UI should provide various **insightful points of view** of your model.
1. The software should provide **versatile export options**.
1. Finally, the CAD application should be **fast**, to support even complex models.


#### 1. Project Health

Before getting into a new CAD program, it's worth considering how it is developed, and how viable that development is long-term. While many people make a whole lifestyle out of never upgrading anything, I'm firmly of the opinion that life is change, and to thrive we have to adapt. Doing that is much harder when using stagnant software.

There are a few sub-criteria that I judge the health of a software project by:

- Is it **open source**? There is very good proprietary software too, but open source software can [provide many benefits](/notes/open-source-benefits).
- Is it **actively developed**? Without ongoing development, you might run into unfixable limitations sooner or later.
- Does it have a **vibrant community** of users and developers? The more people are involved in a constructive way, the higher the likelihood that the project stays viable in the long term.
- Are **documentation** and other learning materials available? In a pinch, well-written source code might do, if you can read it, but having the selection of a wide variety of materials is much better.
- Is the software **packaged** for a wide variety of platforms? If you have to compile a project from source or, even worse, it doesn't support your system at all, this can cause problems that you're not well-equipped to solve.


#### 2. Models Defined as Code

I believe that CAD models [should be defined as code](/notes/code-cad-advantages). But that alone isn't enough. The language that this code is written in should fulfil several criteria:

1. It should be **easy to use**. Some general-purpose programming languages are very complex, syntactically heavy, or cumbersome (sometimes for good reasons). I don't think a lot of complexity is needed for this particular use case though. The language should put few hurdles in front of the CAD designer as is practical.
1. At the same time, the language shouldn't be simplistic. Some models still benefit from non-trivial algorithms, and the language should be **powerful** enough to support that.
1. Programming is an error-prone activity. To alleviate that, the language should be **robust**, meaning it should prevent as many errors as possible. 

These three attributes can be in conflict with one another. Lots of Code-CAD programs use dynamically types languages that are easy to use and powerful, but not robust. Languages that are robust are often not easy to use.

Check out my [note about Code-CAD software](/notes/programmatic-cad) for an overview over existing solutions.


#### 3. Modeling Features

There are some modeling features that I consider crucial to have:

- **Constructive Solid Geometry** is a convenient way to define geometry, and should be available in all CAD programs. It actually is available in all that I've tried.
- Another critical modeling feature are **extrude/sweep** operations. At least, it should be possible to extrude 2D sketches along straight and round/helical paths. Being able to extrude along arbitrary, mathematically defined, paths is very desirable.
- **Referencing and manipulating existing geometry** can be very useful. Tasks like adding a feature to a specific face can become much easier that way.
- **Constraint-based modeling** means being able to define geometry by defining the relationship between different parts of it. In GUI-based CAD software, constraints enable parametric modeling. In Code-CAD software, constraints are less essential. But in some situations, a few well-placed constraints can save a lot of cumbersome code.
- **Fillets and chamfers** are something that I need all the time. Doing them without dedicated support from the CAD software can be a huge pain.

This list is surely incomplete, and I will expand it as I remember and learn about more features.


#### 4. Convenience and Efficiency

The overall user experience of a CAD program should facilitate a convenient and efficient way of working. This means the UI in general should be easy to use. In Code-CAD applications that come with their own editor, this also applies to the editing experience.

Even in Code-CAD applications, a powerful graphical user interface can be an advantage. I'd like to see a hybrid approach, where code is the source of truth, but the GUI can be used to select features, whose origin is then marked in the code. Or to draw sketches and apply constraints, for which code is then generated.


#### 5. Insightful Points of View

It should be easy to view all different aspects of your model, not just the final result. Like components of an assembly, or intermediate stages of modeling.

This seems to be pretty standard in regular GUI-based CAD software, but not in Code-CAD software.


#### 6. Versatile Export Options

A CAD application that doesn't allow you to export your model to external file formats is of limited use. For use with any kind of CNC machine, the CAD software should support export to the preferred formats for the type of CNC machine (I only have direct experience with 3D printers, for which I prefer 3MF).

Exporting images of the model, for use in external media, should also be supported. As should engineering drawings, which can be very useful to have, if building the modeled item involves manual steps.

Ideally, I'd like to be able to include exports from the program in external documents, like web pages. In the form of interactive views of certain aspects of the model, drawings, or even text documentation from the model. This can be very useful for creating project presentations, assembly instructions, or other material.


#### 7. Performance

Performance is one of those aspects of software that tends to go unnoticed, until it's lacking. When it comes to supporting complex models, or exporting models at a high resolution, it becomes critical that the CAD application is capable of high performance.


### Conclusion

So there it is. The criteria by which I evaluate CAD software. Unfortunately, I haven't found a single CAD application yet that meets all of them. The search continues (and in the meantime, {{ ext_link(text="I'm developing my own", url="https://github.com/hannobraun/fornjot") }}).

If you think I'm missing something, [please let me know](/contact). I'm always interested in learning more about the topics I write about.
